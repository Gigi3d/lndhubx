pub mod dealer_engine;

use utils::xzmq::*;

use crossbeam::channel::bounded;
use dealer_engine::*;
use msgs::dealer::{BankStateRequest, Dealer};
use msgs::*;
use std::time::Instant;
use uuid::Uuid;

use kollider_hedging::KolliderHedgingClient;

pub fn start(settings: DealerEngineSettings, bank_sender: ZmqSocket, bank_recv: ZmqSocket) {
    let (kollider_client_tx, kollider_client_rx) = bounded(2024);

    let ws_client = KolliderHedgingClient::connect(
        &settings.kollider_ws_url,
        &settings.kollider_api_key,
        &settings.kollider_api_secret,
        &settings.kollider_api_passphrase,
        kollider_client_tx,
    )
    .unwrap();

    let mut synth_dealer = DealerEngine::new(settings, ws_client);

    let mut listener = |msg: Message| {
        let payload = bincode::serialize(&msg).unwrap();
        bank_sender.send(payload, 0x00).unwrap();
    };

    let mut last_health_check = Instant::now();
    let mut last_house_keeping = Instant::now();

    // Before we start the main loop we have to have received at lease one bank state message.
    while synth_dealer.last_bank_state_update.is_none() {
        let msg = Message::Dealer(Dealer::BankStateRequest(BankStateRequest { req_id: Uuid::new_v4() }));
        listener(msg);
        if let Ok(frame) = bank_recv.recv_msg(0) {
            if let Ok(message) = bincode::deserialize::<Message>(&frame) {
                synth_dealer.process_msg(message, &mut listener);
            };
        }
    }

    loop {
        if let Ok(frame) = bank_recv.recv_msg(1) {
            if let Ok(message) = bincode::deserialize::<Message>(&frame) {
                synth_dealer.process_msg(message, &mut listener);
            };
        }

        if let Ok(message) = kollider_client_rx.try_recv() {
            synth_dealer.process_msg(message, &mut listener);
        }

        if last_health_check.elapsed().as_secs() > 5 {
            synth_dealer.check_health(&mut listener);
            last_health_check = Instant::now();
        }

        if last_house_keeping.elapsed().as_secs() > 5 {
            last_house_keeping = Instant::now();
            synth_dealer.sweep_excess_funds(&mut listener);
        }

        // if synth_dealer.last_bank_state_update.unwrap().elapsed().as_secs() > 10 {
        //     let msg = Message::Dealer(Dealer::BankStateRequest(BankStateRequest {req_id: Uuid::new_v4()}));
        //     listener(msg);
        // }
    }
}