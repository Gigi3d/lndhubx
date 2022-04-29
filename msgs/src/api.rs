use core_types::*;
use rust_decimal::prelude::*;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceResponseError {
    AccountDoesNotExist,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwapResponseError {
    Invalid,
    CurrencyNotAvailable,
    InvalidQuoteId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuoteResponseError {
    Invalid,
    ServiceNotAvailable,
    CurrencyNotAvailable,
    MarketNotAvailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceRequest {
    pub req_id: RequestId,
    pub uid: UserId,
    pub amount: Option<u64>,
    pub meta: String,
    pub currency: Currency,
    pub account_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceResponse {
    pub req_id: RequestId,
    pub uid: UserId,
    pub payment_request: Option<String>,
    pub currency: Currency,
    pub account_id: Uuid,
    pub error: Option<InvoiceResponseError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub req_id: RequestId,
    pub uid: UserId,
    pub payment_request: String,
    pub currency: Currency,
    pub amount: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentResponseError {
    InsufficientFunds,
    InvoiceAlreadyPaid,
    SelfPayment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub req_id: RequestId,
    pub uid: UserId,
    pub success: bool,
    pub currency: Currency,
    pub payment_request: String,
    pub error: Option<PaymentResponseError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapRequest {
    pub req_id: RequestId,
    pub uid: UserId,
    pub amount: Decimal,
    pub from: Currency,
    pub to: Currency,
    pub quote_id: Option<u128>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapResponse {
    pub req_id: RequestId,
    pub uid: UserId,
    pub success: bool,
    pub amount: Decimal,
    pub from: Currency,
    pub to: Currency,
    pub rate: Option<Decimal>,
    pub error: Option<SwapResponseError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalances {
    pub req_id: RequestId,
    pub uid: UserId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balances {
    pub req_id: RequestId,
    pub uid: UserId,
    pub accounts: HashMap<AccountId, Account>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteRequest {
    pub req_id: RequestId,
    pub uid: UserId,
    pub amount: Decimal,
    pub from: Currency,
    pub to: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub req_id: RequestId,
    pub uid: UserId,
    pub amount: Decimal,
    pub from: Currency,
    pub to: Currency,
    // epoch in ms
    pub valid_until: u64,
    pub rate: Option<Decimal>,
    pub quote_id: Option<u128>,
    pub error: Option<QuoteResponseError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Api {
    InvoiceRequest(InvoiceRequest),
    InvoiceResponse(InvoiceResponse),
    PaymentRequest(PaymentRequest),
    PaymentResponse(PaymentResponse),
    SwapRequest(SwapRequest),
    SwapResponse(SwapResponse),
    GetBalances(GetBalances),
    Balances(Balances),
    QuoteRequest(QuoteRequest),
    QuoteResponse(QuoteResponse),
}