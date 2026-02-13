// Copyright 2019-2026 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use serde::de::DeserializeOwned;
use chrono::{DateTime, Utc};

// APICredentials models gateway credentials.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct APICredentials {
    #[serde(rename = "apiKey")]
	pub api_key: String,
    #[serde(rename = "bearerToken")]
    pub bearer_token: String,
    #[serde(rename = "signingKey")]
    pub signing_key: String,
}

// CardType is used to differentiate credit, debit, and EBT.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)] 
pub enum CardType {
    #[default]
    Credit,
    Debit,
    EBT,
    BlockchainGift,
    Healthcare,
}

// SignatureFormat is used to specify the output format for customer signature images.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum SignatureFormat {
    #[serde(rename = "")]
    #[default]
    None,
    #[serde(rename = "png")]
    PNG,
    #[serde(rename = "jpg")]
    JPG,
    #[serde(rename = "gif")]
    GIF,
}

impl SignatureFormat {
    // Function to get the string representation of each variant
    pub fn as_str(&self) -> &'static str {
        match self {
            SignatureFormat::None => "",
            SignatureFormat::PNG => "png",
            SignatureFormat::JPG => "jpg",
            SignatureFormat::GIF => "gif",
        }
    }
}

// CVMType designates a customer verification method.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum CVMType {
    #[serde(rename = "Signature")]
    Signature,
    #[serde(rename = "Offline PIN")]
    OfflinePIN,
    #[serde(rename = "Online PIN")]
    OnlinePIN,
    #[serde(rename = "CDCVM")]
    CDCVM,
    #[serde(rename = "No CVM")]
    NoCVM,
    #[serde(other)]
    #[default]
    Default,
}

impl CVMType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CVMType::Signature => "Signature",
            CVMType::OfflinePIN => "Offline PIN",
            CVMType::OnlinePIN => "Online PIN",
            CVMType::CDCVM => "CDCVM",
            CVMType::NoCVM => "No CVM",
            CVMType::Default => "",
        }
    }
}

// PromptType is used to specify the type of text input data being requested
// from a customer.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum PromptType {
    #[serde(rename = "amount")]
    Amount,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "customer-number")]
    CustomerNumber,
    #[serde(rename = "rewards-number")]
    RewardsNumber,
    #[serde(rename = "first-name")]
    FirstName,
    #[serde(rename = "last-name")]
    LastName,
    #[serde(other)]
    #[default]
    Default,
}

impl PromptType {
    // Function to get the string representation of each variant
    pub fn as_str(&self) -> &'static str {
        match self {
            PromptType::Amount => "amount",
            PromptType::Email => "email",
            PromptType::Phone => "phone",
            PromptType::CustomerNumber => "customer-number",
            PromptType::RewardsNumber => "rewards-number",
            PromptType::FirstName => "first-name",
            PromptType::LastName => "last-name",
            PromptType::Default => "",
        }
    }
}

// AVSResponse indicates the result of address verification.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum AVSResponse {
    #[serde(rename = "")]
    NotApplicable,
    #[serde(rename = "not_supported")]
    NotSupported,
    #[serde(rename = "retry")]
    Retry,
    #[serde(rename = "no_match")]
    NoMatch,
    #[serde(rename = "address_match")]
    AddressMatch,
    #[serde(rename = "zip_match")]
    PostalCodeMatch,
    #[serde(rename = "match")]
    AddressAndPostalCodeMatch,
    #[serde(other)]
    #[default]
    Default,
}

impl AVSResponse {
    // Function to get the string representation of each variant
    pub fn as_str(&self) -> &'static str {
        match self {
            AVSResponse::NotApplicable => "",
            AVSResponse::NotSupported => "not_supported",
            AVSResponse::Retry => "retry",
            AVSResponse::NoMatch => "no_match",
            AVSResponse::AddressMatch => "address_match",
            AVSResponse::PostalCodeMatch => "zip_match",
            AVSResponse::AddressAndPostalCodeMatch => "match",
            AVSResponse::Default => "",
        }
    }
}

// HealthcareType is a category of healthcare.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthcareType {
    Healthcare,
    Prescription,
    Vision,
    Clinic,
    Dental,
    #[default]
    Default,
}

impl HealthcareType {
    // Function to get the string representation of each variant
    pub fn as_str(&self) -> &'static str {
        match self {
            HealthcareType::Healthcare => "healthcare",
            HealthcareType::Prescription => "prescription",
            HealthcareType::Vision => "vision",
            HealthcareType::Clinic => "clinic",
            HealthcareType::Dental => "dental",
            HealthcareType::Default => "",
        }
    }
}

// RoundingMode indicates how partial penny rounding operations should work
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoundingMode {
    Up,
    Nearest,
    Down,
    #[default]
    Default,
}

impl RoundingMode {
    // Function to get the string representation of each variant
    pub fn as_str(&self) -> &'static str {
        match self {
            RoundingMode::Up => "up",
            RoundingMode::Nearest => "nearest",
            RoundingMode::Down => "down",
            RoundingMode::Default => "",
        }
    }
}


/// EMV fields we recommend developers put on their receipts.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReceiptSuggestions {
    /// The EMV Application Identifier.
    #[serde(rename = "aid", default)]
    pub aid: String,
    /// The EMV Application Request Cryptogram.
    #[serde(rename = "arqc", default)]
    pub arqc: String,
    /// The EMV Issuer Application Data.
    #[serde(rename = "iad", default)]
    pub iad: String,
    /// The EMV Authorization Response Code.
    #[serde(rename = "arc", default)]
    pub arc: String,
    /// The EMV Transaction Certificate.
    #[serde(rename = "tc", default)]
    pub tc: String,
    /// The EMV Terminal Verification Response.
    #[serde(rename = "tvr", default)]
    pub tvr: String,
    /// The EMV Transaction Status Indicator.
    #[serde(rename = "tsi", default)]
    pub tsi: String,
    /// The ID of the payment terminal.
    #[serde(rename = "terminalId", default)]
    pub terminal_id: String,
    /// The name of the merchant's business.
    #[serde(rename = "merchantName", default)]
    pub merchant_name: String,
    /// The ID of the merchant.
    #[serde(rename = "merchantId", default)]
    pub merchant_id: String,
    /// The partially masked merchant key required on EMV receipts.
    #[serde(rename = "merchantKey", default)]
    pub merchant_key: String,
    /// A description of the selected AID.
    #[serde(rename = "applicationLabel", default)]
    pub application_label: String,
    /// That the receipt should contain a signature line.
    #[serde(rename = "requestSignature")]
    pub request_signature: bool,
    /// The masked primary account number of the payment card, as required.
    #[serde(rename = "maskedPan", default)]
    pub masked_pan: String,
    /// The amount authorized by the payment network. Could be less than the requested amount
/// for partial auth.
    #[serde(rename = "authorizedAmount")]
    pub authorized_amount: String,
    /// The type of transaction performed (CHARGE, PREAUTH, REFUND, etc).
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The method by which the payment card was entered (MSR, CHIP, KEYED, etc.).
    #[serde(rename = "entryMethod", default)]
    pub entry_method: String,
    /// That PIN verification was performed.
    #[serde(rename = "pinVerified", default)]
    pub pin_verified: bool,
    /// The customer verification method used for the transaction.
    #[serde(rename = "cvmUsed", default)]
    pub cvm_used: CVMType,
    /// That a chip read failure caused the transaction to fall back to the magstripe.
    #[serde(rename = "fallback", default)]
    pub fallback: bool,
    /// The sequence of the transaction in the batch.
    #[serde(rename = "batchSequence", default)]
    pub batch_sequence: i32,
    /// The amount of cash back that was approved.
    #[serde(rename = "cashBackAmount", default)]
    pub cash_back_amount: String,
    /// The amount added to the transaction to cover eligible credit card fees.
    #[serde(rename = "surcharge", default)]
    pub surcharge: String,
    /// The discount applied to the transaction for payment methods ineligible for
/// surcharges.
    #[serde(rename = "cashDiscount", default)]
    pub cash_discount: String,

}

/// A basic api acknowledgement.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Acknowledgement {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,

}

/// A request for customer signature data.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CaptureSignatureRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// A location on the filesystem which a customer signature should be written to.
    #[serde(rename = "sigFile", default)]
    pub sig_file: String,
    /// The image format to be used for returning signatures.
    #[serde(rename = "sigFormat", default)]
    pub sig_format: SignatureFormat,
    /// The width that the signature image should be scaled to, preserving the aspect ratio. If
/// not provided, the signature is returned in the terminal's max resolution.
    #[serde(rename = "sigWidth", default)]
    pub sig_width: i32,
    /// Whether or not signature prompt should be skipped on the terminal. The terminal will
/// indicate whether or not a signature is required by the card in the receipt suggestions
/// response.
    #[serde(rename = "disableSignature", default)]
    pub disable_signature: bool,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,

}

/// Customer signature data.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CaptureSignatureResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The hex encoded signature data.
    #[serde(rename = "sigFile", default)]
    pub sig_file: String,

}

/// Information needed to test connectivity with a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PingRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,

}

/// The response to a ping request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PingResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The ID assigned to the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The ID assigned to the batch.
    #[serde(rename = "batchId", default)]
    pub batch_id: String,
    /// The transaction reference string assigned to the transaction request. If no
/// transaction ref was assiged on the request, then the gateway will randomly generate
/// one.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// The type of transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The hash of the last tick block.
    #[serde(rename = "tickBlock")]
    pub tick_block: String,
    /// That the transaction was processed on the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// The ECC signature of the response. Can be used to ensure that it was signed by the
/// terminal and detect man-in-the middle attacks.
    #[serde(rename = "sig", default)]
    pub sig: String,

}

/// Information needed to retrieve location information for a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LocateRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,

}

/// The response to a locate request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LocateResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The ID assigned to the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The ID assigned to the batch.
    #[serde(rename = "batchId", default)]
    pub batch_id: String,
    /// The transaction reference string assigned to the transaction request. If no
/// transaction ref was assiged on the request, then the gateway will randomly generate
/// one.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// The type of transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The hash of the last tick block.
    #[serde(rename = "tickBlock")]
    pub tick_block: String,
    /// That the transaction was processed on the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// The ECC signature of the response. Can be used to ensure that it was signed by the
/// terminal and detect man-in-the middle attacks.
    #[serde(rename = "sig", default)]
    pub sig: String,
    /// The name assigned to the terminal at activation.
    #[serde(rename = "terminalName")]
    pub terminal_name: String,
    /// The local IP address of the terminal.
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    /// Whether or not the terminal is running in cloud relay mode.
    #[serde(rename = "cloudRelay")]
    pub cloud_relay: bool,
    /// The terminal's public key.
    #[serde(rename = "publicKey")]
    pub public_key: String,

}

/// A message to be displayed on the terminal screen.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MessageRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,
    /// The message to be displayed on the terminal.
    #[serde(rename = "message")]
    pub message: String,

}

/// A simple yes no prompt request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BooleanPromptRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,
    /// The preferred caption for the 'yes' button.
    #[serde(rename = "yesCaption")]
    pub yes_caption: String,
    /// The preferred caption for the 'no' button.
    #[serde(rename = "noCaption")]
    pub no_caption: String,
    /// The text to be displayed on the terminal.
    #[serde(rename = "prompt")]
    pub prompt: String,

}

/// A text prompt request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TextPromptRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,
    /// The prompt type (email, phone, etc).
    #[serde(rename = "promptType")]
    pub prompt_type: PromptType,

}

/// Models a customer data request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// BlockChyp assigned customer id.
    #[serde(rename = "customerId")]
    pub customer_id: String,
    /// Optional customer ref that can be used for the client's system's customer id.
    #[serde(rename = "customerRef")]
    pub customer_ref: String,

}

/// Models a customer data response.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The customer record.
    #[serde(rename = "customer")]
    pub customer: Option<Customer>,

}

/// Models a customer data search request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerSearchRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// Search query for customer searches.
    #[serde(rename = "query")]
    pub query: String,

}

/// Models a customer data search request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// Models a customer update request.
    #[serde(rename = "customer")]
    pub customer: Customer,

}

/// Models customer search results.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerSearchResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The customer results matching the search query.
    #[serde(rename = "customers")]
    pub customers: Option<Vec<Customer>>,

}

/// Models a customer record.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// BlockChyp assigned customer id.
    #[serde(rename = "id")]
    pub id: String,
    /// Optional customer ref that can be used for the client's system's customer id.
    #[serde(rename = "customerRef")]
    pub customer_ref: String,
    /// Customer's first name.
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// Customer's last name.
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// Customer's company name.
    #[serde(rename = "companyName")]
    pub company_name: String,
    /// Customer's email address.
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    /// Customer's SMS or mobile number.
    #[serde(rename = "smsNumber")]
    pub sms_number: String,
    /// Model saved payment methods associated with a customer.
    #[serde(rename = "paymentMethods")]
    pub payment_methods: Option<Vec<CustomerToken>>,

}

/// Retrieves token metadata.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TokenMetadataRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The token to retrieve.
    #[serde(rename = "token")]
    pub token: String,

}

/// Models a payment token metadata response.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TokenMetadataResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The token metadata for a given query.
    #[serde(rename = "token")]
    pub token: CustomerToken,
    /// Details about a payment card derived from its BIN/IIN.
    #[serde(rename = "cardMetadata", default)]
    pub card_metadata: Option<CardMetadata>,

}

/// Updates a payment token.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateTokenRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The token to update.
    #[serde(rename = "token")]
    pub token: String,
    /// Bank account holder type (personal or business).
    #[serde(rename = "accountHolderType")]
    pub account_holder_type: String,
    /// Bank account type (checking or saving).
    #[serde(rename = "accountType")]
    pub account_type: String,
    /// Bank name.
    #[serde(rename = "bankName")]
    pub bank_name: String,
    /// Card holder name.
    #[serde(rename = "cardHolderName")]
    pub card_holder_name: String,
    /// Expiry month.
    #[serde(rename = "expiryMonth")]
    pub expiry_month: String,
    /// Expiry year.
    #[serde(rename = "expiryYear")]
    pub expiry_year: String,
    /// Address.
    #[serde(rename = "address")]
    pub address: String,
    /// Postal code.
    #[serde(rename = "postalCode")]
    pub postal_code: String,

}

/// The response to a update token request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateTokenResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The updated token for a given query.
    #[serde(rename = "token")]
    pub token: CustomerToken,

}

/// Models a customer token.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerToken {
    /// BlockChyp assigned customer id.
    #[serde(rename = "token")]
    pub token: String,
    /// Masked primary account number.
    #[serde(rename = "maskedPan")]
    pub masked_pan: String,
    /// Expiration month.
    #[serde(rename = "expiryMonth")]
    pub expiry_month: String,
    /// Expiration month.
    #[serde(rename = "expiryYear")]
    pub expiry_year: String,
    /// Payment type.
    #[serde(rename = "paymentType")]
    pub payment_type: String,
    /// Bank account type (checking, saving).
    #[serde(rename = "accountType")]
    pub account_type: String,
    /// Bank account holder type (personal, business).
    #[serde(rename = "accountHolderType")]
    pub account_holder_type: String,
    /// Bank name.
    #[serde(rename = "bankName")]
    pub bank_name: String,
    /// Routing number.
    #[serde(rename = "routingNumber")]
    pub routing_number: String,
    /// Token hash (generated with a static salt, Merchant ID, Registration Date and PAN.
    #[serde(rename = "tokenHash")]
    pub token_hash: String,
    /// Card bin.
    #[serde(rename = "bin")]
    pub bin: String,
    /// The card postal code.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The card address.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The card country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// The card holder name.
    #[serde(rename = "cardHolderName", default)]
    pub card_holder_name: String,
    /// Models customer records associated with a payment token.
    #[serde(rename = "customers")]
    pub customers: Option<Vec<Customer>>,

}

/// The response to a text prompt request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TextPromptResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The text prompt response.
    #[serde(rename = "response")]
    pub response: String,

}

/// The response to a boolean prompt request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BooleanPromptResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The boolean prompt response.
    #[serde(rename = "response")]
    pub response: bool,

}

/// Shows details about a white listed card.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WhiteListedCard {
    /// The card BIN.
    #[serde(rename = "bin")]
    pub bin: String,
    /// The track 1 data from the card.
    #[serde(rename = "track1")]
    pub track_1: String,
    /// The track 2 data from the card.
    #[serde(rename = "track2")]
    pub track_2: String,
    /// The card primary account number.
    #[serde(rename = "pan")]
    pub pan: String,

}

/// An authorization request for a charge, preauth, or reverse transaction.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The payment token to be used for this transaction. This should be used for recurring
/// transactions.
    #[serde(rename = "token", default)]
    pub token: String,
    /// Track 1 magnetic stripe data.
    #[serde(rename = "track1", default)]
    pub track_1: String,
    /// Track 2 magnetic stripe data.
    #[serde(rename = "track2", default)]
    pub track_2: String,
    /// The primary account number. We recommend using the terminal or e-commerce
/// tokenization libraries instead of passing account numbers in directly, as this would
/// put your application in PCI scope.
    #[serde(rename = "pan", default)]
    pub pan: String,
    /// The ACH routing number for ACH transactions.
    #[serde(rename = "routingNumber", default)]
    pub routing_number: String,
    /// The cardholder name. Only required if the request includes a primary account number or
/// track data.
    #[serde(rename = "cardholderName", default)]
    pub cardholder_name: String,
    /// The card expiration month for use with PAN based transactions.
    #[serde(rename = "expMonth", default)]
    pub exp_month: String,
    /// The card expiration year for use with PAN based transactions.
    #[serde(rename = "expYear", default)]
    pub exp_year: String,
    /// The card CVV for use with PAN based transactions.
    #[serde(rename = "cvv", default)]
    pub cvv: String,
    /// The cardholder address for use with address verification.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The cardholder postal code for use with address verification.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The cardholder country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// That the payment entry method is a manual keyed transaction. If this is true, no other
/// payment method will be accepted.
    #[serde(rename = "manualEntry", default)]
    pub manual_entry: bool,
    /// The key serial number used for DUKPT encryption.
    #[serde(rename = "ksn", default)]
    pub ksn: String,
    /// The encrypted pin block.
    #[serde(rename = "pinBlock", default)]
    pub pin_block: String,
    /// Designates categories of cards: credit, debit, EBT.
    #[serde(rename = "cardType", default)]
    pub card_type: CardType,
    /// Designates brands of payment methods: Visa, Discover, etc.
    #[serde(rename = "paymentType", default)]
    pub payment_type: String,
    /// The transaction currency code.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The requested amount.
    #[serde(rename = "amount")]
    pub amount: String,
    /// That the request is tax exempt. Only required for tax exempt level 2 processing.
    #[serde(rename = "taxExempt")]
    pub tax_exempt: bool,
    /// A flag to add a surcharge to the transaction to cover credit card fees, if permitted.
    #[serde(rename = "surcharge")]
    pub surcharge: bool,
    /// A flag that applies a discount to negate the surcharge for debit transactions or other
/// surcharge ineligible payment methods.
    #[serde(rename = "cashDiscount")]
    pub cash_discount: bool,
    /// A location on the filesystem which a customer signature should be written to.
    #[serde(rename = "sigFile", default)]
    pub sig_file: String,
    /// The image format to be used for returning signatures.
    #[serde(rename = "sigFormat", default)]
    pub sig_format: SignatureFormat,
    /// The width that the signature image should be scaled to, preserving the aspect ratio. If
/// not provided, the signature is returned in the terminal's max resolution.
    #[serde(rename = "sigWidth", default)]
    pub sig_width: i32,
    /// Whether or not signature prompt should be skipped on the terminal. The terminal will
/// indicate whether or not a signature is required by the card in the receipt suggestions
/// response.
    #[serde(rename = "disableSignature", default)]
    pub disable_signature: bool,
    /// The tip amount.
    #[serde(rename = "tipAmount", default)]
    pub tip_amount: String,
    /// The tax amount.
    #[serde(rename = "taxAmount", default)]
    pub tax_amount: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,
    /// Can be used to update a pre-auth to a new amount, sometimes called incremental auth.
    #[serde(rename = "transactionId", default)]
    pub transaction_id: String,
    /// Used to validate online gift card authorizations.
    #[serde(rename = "onlineAuthCode", default)]
    pub online_auth_code: String,
    /// That the payment method should be added to the token vault alongside the
/// authorization.
    #[serde(rename = "enroll", default)]
    pub enroll: bool,
    /// Duplicate detection should be bypassed.
    #[serde(rename = "bypassDupeFilter", default)]
    pub bypass_dupe_filter: bool,
    /// A narrative description of the transaction.
    #[serde(rename = "description", default)]
    pub description: String,
    /// That the terminal should request a tip from the user before starting the transaction.
    #[serde(rename = "promptForTip", default)]
    pub prompt_for_tip: bool,
    /// That cash back should be enabled for supported cards.
    #[serde(rename = "cashBackEnabled", default)]
    pub cash_back_enabled: bool,
    /// That this transaction should be treated as MOTO with a card on file.
    #[serde(rename = "cardOnFile", default)]
    pub card_on_file: bool,
    /// That this transaction should be treated as a recurring transaction.
    #[serde(rename = "recurring", default)]
    pub recurring: bool,
    /// Manually sets the CIT (Customer Initiated Transaction) flag.
    #[serde(rename = "cit", default)]
    pub cit: bool,
    /// Manually sets the MIT (Merchant Initiated Transaction) flag.
    #[serde(rename = "mit", default)]
    pub mit: bool,
    /// That this transaction should be treated as a subscription recurring transaction.
    #[serde(rename = "subscription", default)]
    pub subscription: bool,
    /// The purchase order number, if known.
    #[serde(rename = "purchaseOrderNumber", default)]
    pub purchase_order_number: String,
    /// The supplier reference number, if known.
    #[serde(rename = "supplierReferenceNumber", default)]
    pub supplier_reference_number: String,
    /// An item to display. Can be overwritten or appended, based on the request type.
    #[serde(rename = "lineItems")]
    pub line_items: Option<Vec<TransactionDisplayItem>>,
    /// A map of alternate currencies and the price in each currency. Use only if you want to set
/// your own exchange rate for a crypto transaction.
    #[serde(rename = "altPrices", default)]
    pub alt_prices: HashMap<String, String>,
    /// Customer information.
    #[serde(rename = "customer")]
    pub customer: Option<Customer>,
    /// How partial pennies should be rounded for calculated values like surcharges.
/// Rounding up is the default behavior.
    #[serde(rename = "roundingMode")]
    pub rounding_mode: Option<RoundingMode>,
    /// Details for HSA/FSA transactions.
    #[serde(rename = "healthcareMetadata", default)]
    pub healthcare_metadata: Option<HealthcareMetadata>,
    /// That the transaction should be a cryptocurrency transaction. Value should be a crypto
/// currency code (ETH, BTC) or ANY to prompt the user to choose from supported
/// cryptocurrencies.
    #[serde(rename = "cryptocurrency", default)]
    pub cryptocurrency: Option<String>,
    /// An optional parameter that can be used to force a crypto transaction onto a level one or
/// level two network. Valid values are L1 and L2. Defaults to L1.
    #[serde(rename = "cryptoNetwork", default)]
    pub crypto_network: Option<String>,
    /// Can be used to specify a specific receive address for a crypto transaction. Disabled by
/// default. This should only be used by sophisticated users with access to properly
/// configured hot wallets.
    #[serde(rename = "cryptoReceiveAddress", default)]
    pub crypto_receive_address: Option<String>,
    /// Can optionally add a label to the payment request if the target cryptocurrency
/// supports labels. Defaults to the merchant's DBA Name.
    #[serde(rename = "paymentRequestLabel", default)]
    pub payment_request_label: Option<String>,
    /// Can optionally add a message to the payment request if the target cryptocurrency
/// supports labels. Defaults to empty.
    #[serde(rename = "paymentRequestMessage", default)]
    pub payment_request_message: Option<String>,
    /// Instructs the terminal to simulate a post auth chip rejection that would trigger an
/// automatic reversal.
    #[serde(rename = "simulateChipRejection", default)]
    pub simulate_chip_rejection: bool,
    /// Instructs the terminal to simulate an out of order automatic reversal.
    #[serde(rename = "simulateOutOfOrderReversal", default)]
    pub simulate_out_of_order_reversal: bool,
    /// Causes auto-reversals on the terminal to be executed asyncronously. Use with caution
/// and in conjunction with the transaction status API.
    #[serde(rename = "asyncReversals", default)]
    pub async_reversals: bool,
    /// A passthrough surcharge amount. This surcharge amount will be passed directly to the
/// gateway and is not directly calculated.
    #[serde(rename = "passthroughSurcharge", default)]
    pub passthrough_surcharge: String,
    /// Marks a transaction as HSA/FSA.
    #[serde(rename = "healthcare", default)]
    pub healthcare: bool,
    /// The total amount to process as healthcare.
    #[serde(rename = "healthcareTotal", default)]
    pub healthcare_total: String,
    /// The total amount to process as ebt.
    #[serde(rename = "ebtTotal", default)]
    pub ebt_total: String,
    /// That this transaction will include a card metadata lookup.
    #[serde(rename = "cardMetadataLookup", default)]
    pub card_metadata_lookup: bool,
    /// The total discount amount for the transaction, and will overide additive logic for
/// line item discounts.
    #[serde(rename = "totalDiscountAmount", default)]
    pub total_discount_amount: String,
    /// The shipping cost associated with the transaction.
    #[serde(rename = "shippingAmount", default)]
    pub shipping_amount: String,
    /// The duty amount associated with the transaction.
    #[serde(rename = "dutyAmount", default)]
    pub duty_amount: String,
    /// The processor ID associated with the transaction.
    #[serde(rename = "processorId", default)]
    pub processor_id: String,
    /// The external customer ID associated with the transaction.
    #[serde(rename = "externalCustomerId", default)]
    pub external_customer_id: String,
    /// Three character, numeric, ship-to country code. Defaults to '840' (USA) if not
/// specified.
    #[serde(rename = "destinationCountryCode", default)]
    pub destination_country_code: String,
    /// Nine character postal code for shipping origin addresses. For US addresses, this is a
/// 5+4 ZIP or five digit ZIP.
    #[serde(rename = "shipFromPostalCode", default)]
    pub ship_from_postal_code: String,
    /// Nine character postal code for shipping destination addresses. For US addresses,
/// this is a 5+4 ZIP or five digit ZIP.
    #[serde(rename = "shipToPostalCode", default)]
    pub ship_to_postal_code: String,
    /// The purchase order date.
    #[serde(rename = "orderDate", default)]
    pub order_date: Option<DateTime<Utc>>,

}

/// Essential information about a payment card derived from its BIN/IIN.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CardMetadata {
    /// The brand or network of the card (e.g., Visa, Mastercard, Amex).
    #[serde(rename = "cardBrand")]
    pub card_brand: String,
    /// The name of the financial institution that issued the card.
    #[serde(rename = "issuerName")]
    pub issuer_name: String,
    /// Whether the card supports Level 3 processing for detailed transaction data.
    #[serde(rename = "l3")]
    pub l_3: bool,
    /// Whether the card supports Level 2 processing for additional transaction data.
    #[serde(rename = "l2")]
    pub l_2: bool,
    /// The general category or type of the card product.
    #[serde(rename = "productType")]
    pub product_type: String,
    /// The specific name or designation of the card product.
    #[serde(rename = "productName")]
    pub product_name: String,
    /// Whether the card is an Electronic Benefit Transfer (EBT) card.
    #[serde(rename = "ebt")]
    pub ebt: bool,
    /// Whether the card is a debit card.
    #[serde(rename = "debit")]
    pub debit: bool,
    /// Whether the card is a healthcare-specific payment card.
    #[serde(rename = "healthcare")]
    pub healthcare: bool,
    /// Whether the card is a prepaid card.
    #[serde(rename = "prepaid")]
    pub prepaid: bool,
    /// The geographical region associated with the card's issuer.
    #[serde(rename = "region")]
    pub region: String,
    /// The country associated with the card's issuer.
    #[serde(rename = "country")]
    pub country: String,

}

/// Retrieves card metadata.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CardMetadataRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The payment token to be used for this transaction. This should be used for recurring
/// transactions.
    #[serde(rename = "token", default)]
    pub token: String,
    /// Track 1 magnetic stripe data.
    #[serde(rename = "track1", default)]
    pub track_1: String,
    /// Track 2 magnetic stripe data.
    #[serde(rename = "track2", default)]
    pub track_2: String,
    /// The primary account number. We recommend using the terminal or e-commerce
/// tokenization libraries instead of passing account numbers in directly, as this would
/// put your application in PCI scope.
    #[serde(rename = "pan", default)]
    pub pan: String,
    /// The ACH routing number for ACH transactions.
    #[serde(rename = "routingNumber", default)]
    pub routing_number: String,
    /// The cardholder name. Only required if the request includes a primary account number or
/// track data.
    #[serde(rename = "cardholderName", default)]
    pub cardholder_name: String,
    /// The card expiration month for use with PAN based transactions.
    #[serde(rename = "expMonth", default)]
    pub exp_month: String,
    /// The card expiration year for use with PAN based transactions.
    #[serde(rename = "expYear", default)]
    pub exp_year: String,
    /// The card CVV for use with PAN based transactions.
    #[serde(rename = "cvv", default)]
    pub cvv: String,
    /// The cardholder address for use with address verification.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The cardholder postal code for use with address verification.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The cardholder country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// That the payment entry method is a manual keyed transaction. If this is true, no other
/// payment method will be accepted.
    #[serde(rename = "manualEntry", default)]
    pub manual_entry: bool,
    /// The key serial number used for DUKPT encryption.
    #[serde(rename = "ksn", default)]
    pub ksn: String,
    /// The encrypted pin block.
    #[serde(rename = "pinBlock", default)]
    pub pin_block: String,
    /// Designates categories of cards: credit, debit, EBT.
    #[serde(rename = "cardType", default)]
    pub card_type: CardType,
    /// Designates brands of payment methods: Visa, Discover, etc.
    #[serde(rename = "paymentType", default)]
    pub payment_type: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,
    /// Marks a transaction as HSA/FSA.
    #[serde(rename = "healthcare", default)]
    pub healthcare: bool,

}

/// The response to a card metadata request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CardMetadataResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The payment token, if the payment was enrolled in the vault.
    #[serde(rename = "token", default)]
    pub token: String,
    /// The entry method for the transaction (CHIP, MSR, KEYED, etc).
    #[serde(rename = "entryMethod", default)]
    pub entry_method: String,
    /// The card brand (VISA, MC, AMEX, DEBIT, etc).
    #[serde(rename = "paymentType", default)]
    pub payment_type: String,
    /// Provides network level detail on how a transaction was routed, especially for debit
/// transactions.
    #[serde(rename = "network", default)]
    pub network: String,
    /// Identifies the card association based on bin number. Used primarily used to indicate
/// the major logo on a card, even when debit transactions are routed on a different
/// network.
    #[serde(rename = "logo", default)]
    pub logo: String,
    /// The masked primary account number.
    #[serde(rename = "maskedPan", default)]
    pub masked_pan: String,
    /// The BlockChyp public key if the user presented a BlockChyp payment card.
    #[serde(rename = "publicKey", default)]
    pub public_key: String,
    /// That the transaction did something that would put the system in PCI scope.
    #[serde(rename = "ScopeAlert", default)]
    pub scope_alert: bool,
    /// The cardholder name.
    #[serde(rename = "cardHolder", default)]
    pub card_holder: String,
    /// The card expiration month in MM format.
    #[serde(rename = "expMonth", default)]
    pub exp_month: String,
    /// The card expiration year in YY format.
    #[serde(rename = "expYear", default)]
    pub exp_year: String,
    /// The card postal code.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The card address.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The card country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// Address verification results if address information was submitted.
    #[serde(rename = "avsResponse")]
    pub avs_response: AVSResponse,
    /// The CVV verification result if CVV was submitted.
    #[serde(rename = "cvvResponse", default)]
    pub cvv_response: String,
    /// Suggested receipt fields.
    #[serde(rename = "receiptSuggestions")]
    pub receipt_suggestions: ReceiptSuggestions,
    /// Customer data, if any. Preserved for reverse compatibility.
    #[serde(rename = "customer")]
    pub customer: Option<Customer>,
    /// Customer data, if any.
    #[serde(rename = "customers")]
    pub customers: Option<Vec<Customer>>,
    /// Details about a payment card derived from its BIN/IIN.
    #[serde(rename = "cardMetadata", default)]
    pub card_metadata: Option<CardMetadata>,

}

/// A request for the remaining balance on a payment type.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BalanceRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The payment token to be used for this transaction. This should be used for recurring
/// transactions.
    #[serde(rename = "token", default)]
    pub token: String,
    /// Track 1 magnetic stripe data.
    #[serde(rename = "track1", default)]
    pub track_1: String,
    /// Track 2 magnetic stripe data.
    #[serde(rename = "track2", default)]
    pub track_2: String,
    /// The primary account number. We recommend using the terminal or e-commerce
/// tokenization libraries instead of passing account numbers in directly, as this would
/// put your application in PCI scope.
    #[serde(rename = "pan", default)]
    pub pan: String,
    /// The ACH routing number for ACH transactions.
    #[serde(rename = "routingNumber", default)]
    pub routing_number: String,
    /// The cardholder name. Only required if the request includes a primary account number or
/// track data.
    #[serde(rename = "cardholderName", default)]
    pub cardholder_name: String,
    /// The card expiration month for use with PAN based transactions.
    #[serde(rename = "expMonth", default)]
    pub exp_month: String,
    /// The card expiration year for use with PAN based transactions.
    #[serde(rename = "expYear", default)]
    pub exp_year: String,
    /// The card CVV for use with PAN based transactions.
    #[serde(rename = "cvv", default)]
    pub cvv: String,
    /// The cardholder address for use with address verification.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The cardholder postal code for use with address verification.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The cardholder country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// That the payment entry method is a manual keyed transaction. If this is true, no other
/// payment method will be accepted.
    #[serde(rename = "manualEntry", default)]
    pub manual_entry: bool,
    /// The key serial number used for DUKPT encryption.
    #[serde(rename = "ksn", default)]
    pub ksn: String,
    /// The encrypted pin block.
    #[serde(rename = "pinBlock", default)]
    pub pin_block: String,
    /// Designates categories of cards: credit, debit, EBT.
    #[serde(rename = "cardType", default)]
    pub card_type: CardType,
    /// Designates brands of payment methods: Visa, Discover, etc.
    #[serde(rename = "paymentType", default)]
    pub payment_type: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,

}

/// The response to a balance request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BalanceResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The ID assigned to the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The ID assigned to the batch.
    #[serde(rename = "batchId", default)]
    pub batch_id: String,
    /// The transaction reference string assigned to the transaction request. If no
/// transaction ref was assiged on the request, then the gateway will randomly generate
/// one.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// The type of transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The hash of the last tick block.
    #[serde(rename = "tickBlock")]
    pub tick_block: String,
    /// That the transaction was processed on the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// The ECC signature of the response. Can be used to ensure that it was signed by the
/// terminal and detect man-in-the middle attacks.
    #[serde(rename = "sig", default)]
    pub sig: String,
    /// The payment token, if the payment was enrolled in the vault.
    #[serde(rename = "token", default)]
    pub token: String,
    /// The entry method for the transaction (CHIP, MSR, KEYED, etc).
    #[serde(rename = "entryMethod", default)]
    pub entry_method: String,
    /// The card brand (VISA, MC, AMEX, DEBIT, etc).
    #[serde(rename = "paymentType", default)]
    pub payment_type: String,
    /// Provides network level detail on how a transaction was routed, especially for debit
/// transactions.
    #[serde(rename = "network", default)]
    pub network: String,
    /// Identifies the card association based on bin number. Used primarily used to indicate
/// the major logo on a card, even when debit transactions are routed on a different
/// network.
    #[serde(rename = "logo", default)]
    pub logo: String,
    /// The masked primary account number.
    #[serde(rename = "maskedPan", default)]
    pub masked_pan: String,
    /// The BlockChyp public key if the user presented a BlockChyp payment card.
    #[serde(rename = "publicKey", default)]
    pub public_key: String,
    /// That the transaction did something that would put the system in PCI scope.
    #[serde(rename = "ScopeAlert", default)]
    pub scope_alert: bool,
    /// The cardholder name.
    #[serde(rename = "cardHolder", default)]
    pub card_holder: String,
    /// The card expiration month in MM format.
    #[serde(rename = "expMonth", default)]
    pub exp_month: String,
    /// The card expiration year in YY format.
    #[serde(rename = "expYear", default)]
    pub exp_year: String,
    /// The card postal code.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The card address.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The card country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// Address verification results if address information was submitted.
    #[serde(rename = "avsResponse")]
    pub avs_response: AVSResponse,
    /// The CVV verification result if CVV was submitted.
    #[serde(rename = "cvvResponse", default)]
    pub cvv_response: String,
    /// Suggested receipt fields.
    #[serde(rename = "receiptSuggestions")]
    pub receipt_suggestions: ReceiptSuggestions,
    /// Customer data, if any. Preserved for reverse compatibility.
    #[serde(rename = "customer")]
    pub customer: Option<Customer>,
    /// Customer data, if any.
    #[serde(rename = "customers")]
    pub customers: Option<Vec<Customer>>,
    /// Remaining balance on the payment method.
    #[serde(rename = "remainingBalance", default)]
    pub remaining_balance: String,

}

/// A refund request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The payment token to be used for this transaction. This should be used for recurring
/// transactions.
    #[serde(rename = "token", default)]
    pub token: String,
    /// Track 1 magnetic stripe data.
    #[serde(rename = "track1", default)]
    pub track_1: String,
    /// Track 2 magnetic stripe data.
    #[serde(rename = "track2", default)]
    pub track_2: String,
    /// The primary account number. We recommend using the terminal or e-commerce
/// tokenization libraries instead of passing account numbers in directly, as this would
/// put your application in PCI scope.
    #[serde(rename = "pan", default)]
    pub pan: String,
    /// The ACH routing number for ACH transactions.
    #[serde(rename = "routingNumber", default)]
    pub routing_number: String,
    /// The cardholder name. Only required if the request includes a primary account number or
/// track data.
    #[serde(rename = "cardholderName", default)]
    pub cardholder_name: String,
    /// The card expiration month for use with PAN based transactions.
    #[serde(rename = "expMonth", default)]
    pub exp_month: String,
    /// The card expiration year for use with PAN based transactions.
    #[serde(rename = "expYear", default)]
    pub exp_year: String,
    /// The card CVV for use with PAN based transactions.
    #[serde(rename = "cvv", default)]
    pub cvv: String,
    /// The cardholder address for use with address verification.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The cardholder postal code for use with address verification.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The cardholder country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// That the payment entry method is a manual keyed transaction. If this is true, no other
/// payment method will be accepted.
    #[serde(rename = "manualEntry", default)]
    pub manual_entry: bool,
    /// The key serial number used for DUKPT encryption.
    #[serde(rename = "ksn", default)]
    pub ksn: String,
    /// The encrypted pin block.
    #[serde(rename = "pinBlock", default)]
    pub pin_block: String,
    /// Designates categories of cards: credit, debit, EBT.
    #[serde(rename = "cardType", default)]
    pub card_type: CardType,
    /// Designates brands of payment methods: Visa, Discover, etc.
    #[serde(rename = "paymentType", default)]
    pub payment_type: String,
    /// The ID of the previous transaction being referenced.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The transaction currency code.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The requested amount.
    #[serde(rename = "amount")]
    pub amount: String,
    /// That the request is tax exempt. Only required for tax exempt level 2 processing.
    #[serde(rename = "taxExempt")]
    pub tax_exempt: bool,
    /// A flag to add a surcharge to the transaction to cover credit card fees, if permitted.
    #[serde(rename = "surcharge")]
    pub surcharge: bool,
    /// A flag that applies a discount to negate the surcharge for debit transactions or other
/// surcharge ineligible payment methods.
    #[serde(rename = "cashDiscount")]
    pub cash_discount: bool,
    /// A location on the filesystem which a customer signature should be written to.
    #[serde(rename = "sigFile", default)]
    pub sig_file: String,
    /// The image format to be used for returning signatures.
    #[serde(rename = "sigFormat", default)]
    pub sig_format: SignatureFormat,
    /// The width that the signature image should be scaled to, preserving the aspect ratio. If
/// not provided, the signature is returned in the terminal's max resolution.
    #[serde(rename = "sigWidth", default)]
    pub sig_width: i32,
    /// Whether or not signature prompt should be skipped on the terminal. The terminal will
/// indicate whether or not a signature is required by the card in the receipt suggestions
/// response.
    #[serde(rename = "disableSignature", default)]
    pub disable_signature: bool,
    /// The tip amount.
    #[serde(rename = "tipAmount", default)]
    pub tip_amount: String,
    /// The tax amount.
    #[serde(rename = "taxAmount", default)]
    pub tax_amount: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,
    /// Details for HSA/FSA transactions.
    #[serde(rename = "healthcareMetadata", default)]
    pub healthcare_metadata: Option<HealthcareMetadata>,
    /// Instructs the terminal to simulate a post auth chip rejection that would trigger an
/// automatic reversal.
    #[serde(rename = "simulateChipRejection", default)]
    pub simulate_chip_rejection: bool,
    /// Instructs the terminal to simulate an out of order automatic reversal.
    #[serde(rename = "simulateOutOfOrderReversal", default)]
    pub simulate_out_of_order_reversal: bool,
    /// Causes auto-reversals on the terminal to be executed asyncronously. Use with caution
/// and in conjunction with the transaction status API.
    #[serde(rename = "asyncReversals", default)]
    pub async_reversals: bool,
    /// Manually sets the CIT (Customer Initiated Transaction) flag.
    #[serde(rename = "cit", default)]
    pub cit: bool,
    /// Manually sets the MIT (Merchant Initiated Transaction) flag.
    #[serde(rename = "mit", default)]
    pub mit: bool,
    /// That this transaction will include a card metadata lookup.
    #[serde(rename = "cardMetadataLookup", default)]
    pub card_metadata_lookup: bool,

}

/// The information needed to capture a preauth.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CaptureRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The ID of the previous transaction being referenced.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The transaction currency code.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The requested amount.
    #[serde(rename = "amount")]
    pub amount: String,
    /// That the request is tax exempt. Only required for tax exempt level 2 processing.
    #[serde(rename = "taxExempt")]
    pub tax_exempt: bool,
    /// A flag to add a surcharge to the transaction to cover credit card fees, if permitted.
    #[serde(rename = "surcharge")]
    pub surcharge: bool,
    /// A flag that applies a discount to negate the surcharge for debit transactions or other
/// surcharge ineligible payment methods.
    #[serde(rename = "cashDiscount")]
    pub cash_discount: bool,
    /// The tip amount.
    #[serde(rename = "tipAmount", default)]
    pub tip_amount: String,
    /// The tax amount.
    #[serde(rename = "taxAmount", default)]
    pub tax_amount: String,
    /// The number of shipments the original authorization will be broken into.
    #[serde(rename = "shipmentCount")]
    pub shipment_count: i32,
    /// Which shipment this particular capture is for.
    #[serde(rename = "shipmentNumber")]
    pub shipment_number: i32,

}

/// The response to a capture request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CaptureResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// That the transaction was approved.
    #[serde(rename = "approved")]
    pub approved: bool,
    /// The auth code from the payment network.
    #[serde(rename = "authCode", default)]
    pub auth_code: String,
    /// The code returned by the terminal or the card issuer to indicate the disposition of the
/// message.
    #[serde(rename = "authResponseCode", default)]
    pub auth_response_code: String,
    /// The ID assigned to the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The ID assigned to the batch.
    #[serde(rename = "batchId", default)]
    pub batch_id: String,
    /// The transaction reference string assigned to the transaction request. If no
/// transaction ref was assiged on the request, then the gateway will randomly generate
/// one.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// The type of transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The hash of the last tick block.
    #[serde(rename = "tickBlock")]
    pub tick_block: String,
    /// That the transaction was processed on the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// The ECC signature of the response. Can be used to ensure that it was signed by the
/// terminal and detect man-in-the middle attacks.
    #[serde(rename = "sig", default)]
    pub sig: String,
    /// Whether or not the transaction was approved for a partial amount.
    #[serde(rename = "partialAuth")]
    pub partial_auth: bool,
    /// Whether or not an alternate currency was used.
    #[serde(rename = "altCurrency")]
    pub alt_currency: bool,
    /// Whether or not a request was settled on an FSA card.
    #[serde(rename = "fsaAuth")]
    pub fsa_auth: bool,
    /// The currency code used for the transaction.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The requested amount.
    #[serde(rename = "requestedAmount")]
    pub requested_amount: String,
    /// The authorized amount. May not match the requested amount in the event of a partial
/// auth.
    #[serde(rename = "authorizedAmount")]
    pub authorized_amount: String,
    /// The remaining balance on the payment method.
    #[serde(rename = "remainingBalance")]
    pub remaining_balance: String,
    /// The tip amount.
    #[serde(rename = "tipAmount")]
    pub tip_amount: String,
    /// The tax amount.
    #[serde(rename = "taxAmount")]
    pub tax_amount: String,
    /// The cash back amount the customer requested during the transaction.
    #[serde(rename = "requestedCashBackAmount")]
    pub requested_cash_back_amount: String,
    /// The amount of cash back authorized by the gateway. This amount will be the entire amount
/// requested, or zero.
    #[serde(rename = "authorizedCashBackAmount")]
    pub authorized_cash_back_amount: String,
    /// The payment token, if the payment was enrolled in the vault.
    #[serde(rename = "token", default)]
    pub token: String,
    /// The entry method for the transaction (CHIP, MSR, KEYED, etc).
    #[serde(rename = "entryMethod", default)]
    pub entry_method: String,
    /// The card brand (VISA, MC, AMEX, DEBIT, etc).
    #[serde(rename = "paymentType", default)]
    pub payment_type: String,
    /// Provides network level detail on how a transaction was routed, especially for debit
/// transactions.
    #[serde(rename = "network", default)]
    pub network: String,
    /// Identifies the card association based on bin number. Used primarily used to indicate
/// the major logo on a card, even when debit transactions are routed on a different
/// network.
    #[serde(rename = "logo", default)]
    pub logo: String,
    /// The masked primary account number.
    #[serde(rename = "maskedPan", default)]
    pub masked_pan: String,
    /// The BlockChyp public key if the user presented a BlockChyp payment card.
    #[serde(rename = "publicKey", default)]
    pub public_key: String,
    /// That the transaction did something that would put the system in PCI scope.
    #[serde(rename = "ScopeAlert", default)]
    pub scope_alert: bool,
    /// The cardholder name.
    #[serde(rename = "cardHolder", default)]
    pub card_holder: String,
    /// The card expiration month in MM format.
    #[serde(rename = "expMonth", default)]
    pub exp_month: String,
    /// The card expiration year in YY format.
    #[serde(rename = "expYear", default)]
    pub exp_year: String,
    /// The card postal code.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The card address.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The card country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// Address verification results if address information was submitted.
    #[serde(rename = "avsResponse")]
    pub avs_response: AVSResponse,
    /// The CVV verification result if CVV was submitted.
    #[serde(rename = "cvvResponse", default)]
    pub cvv_response: String,
    /// Suggested receipt fields.
    #[serde(rename = "receiptSuggestions")]
    pub receipt_suggestions: ReceiptSuggestions,
    /// Customer data, if any. Preserved for reverse compatibility.
    #[serde(rename = "customer")]
    pub customer: Option<Customer>,
    /// Customer data, if any.
    #[serde(rename = "customers")]
    pub customers: Option<Vec<Customer>>,

}

/// A void request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VoidRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The ID of the previous transaction being referenced.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,

}

/// The response to a void request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VoidResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// That the transaction was approved.
    #[serde(rename = "approved")]
    pub approved: bool,
    /// The auth code from the payment network.
    #[serde(rename = "authCode", default)]
    pub auth_code: String,
    /// The code returned by the terminal or the card issuer to indicate the disposition of the
/// message.
    #[serde(rename = "authResponseCode", default)]
    pub auth_response_code: String,
    /// The ID assigned to the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The ID assigned to the batch.
    #[serde(rename = "batchId", default)]
    pub batch_id: String,
    /// The transaction reference string assigned to the transaction request. If no
/// transaction ref was assiged on the request, then the gateway will randomly generate
/// one.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// The type of transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The hash of the last tick block.
    #[serde(rename = "tickBlock")]
    pub tick_block: String,
    /// That the transaction was processed on the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// The ECC signature of the response. Can be used to ensure that it was signed by the
/// terminal and detect man-in-the middle attacks.
    #[serde(rename = "sig", default)]
    pub sig: String,
    /// The payment token, if the payment was enrolled in the vault.
    #[serde(rename = "token", default)]
    pub token: String,
    /// The entry method for the transaction (CHIP, MSR, KEYED, etc).
    #[serde(rename = "entryMethod", default)]
    pub entry_method: String,
    /// The card brand (VISA, MC, AMEX, DEBIT, etc).
    #[serde(rename = "paymentType", default)]
    pub payment_type: String,
    /// Provides network level detail on how a transaction was routed, especially for debit
/// transactions.
    #[serde(rename = "network", default)]
    pub network: String,
    /// Identifies the card association based on bin number. Used primarily used to indicate
/// the major logo on a card, even when debit transactions are routed on a different
/// network.
    #[serde(rename = "logo", default)]
    pub logo: String,
    /// The masked primary account number.
    #[serde(rename = "maskedPan", default)]
    pub masked_pan: String,
    /// The BlockChyp public key if the user presented a BlockChyp payment card.
    #[serde(rename = "publicKey", default)]
    pub public_key: String,
    /// That the transaction did something that would put the system in PCI scope.
    #[serde(rename = "ScopeAlert", default)]
    pub scope_alert: bool,
    /// The cardholder name.
    #[serde(rename = "cardHolder", default)]
    pub card_holder: String,
    /// The card expiration month in MM format.
    #[serde(rename = "expMonth", default)]
    pub exp_month: String,
    /// The card expiration year in YY format.
    #[serde(rename = "expYear", default)]
    pub exp_year: String,
    /// The card postal code.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The card address.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The card country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// Address verification results if address information was submitted.
    #[serde(rename = "avsResponse")]
    pub avs_response: AVSResponse,
    /// The CVV verification result if CVV was submitted.
    #[serde(rename = "cvvResponse", default)]
    pub cvv_response: String,
    /// Suggested receipt fields.
    #[serde(rename = "receiptSuggestions")]
    pub receipt_suggestions: ReceiptSuggestions,
    /// Customer data, if any. Preserved for reverse compatibility.
    #[serde(rename = "customer")]
    pub customer: Option<Customer>,
    /// Customer data, if any.
    #[serde(rename = "customers")]
    pub customers: Option<Vec<Customer>>,
    /// The hex encoded signature data.
    #[serde(rename = "sigFile", default)]
    pub sig_file: String,

}

/// The information needed to enroll a new payment method in the token vault.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnrollRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The payment token to be used for this transaction. This should be used for recurring
/// transactions.
    #[serde(rename = "token", default)]
    pub token: String,
    /// Track 1 magnetic stripe data.
    #[serde(rename = "track1", default)]
    pub track_1: String,
    /// Track 2 magnetic stripe data.
    #[serde(rename = "track2", default)]
    pub track_2: String,
    /// The primary account number. We recommend using the terminal or e-commerce
/// tokenization libraries instead of passing account numbers in directly, as this would
/// put your application in PCI scope.
    #[serde(rename = "pan", default)]
    pub pan: String,
    /// The ACH routing number for ACH transactions.
    #[serde(rename = "routingNumber", default)]
    pub routing_number: String,
    /// The cardholder name. Only required if the request includes a primary account number or
/// track data.
    #[serde(rename = "cardholderName", default)]
    pub cardholder_name: String,
    /// The card expiration month for use with PAN based transactions.
    #[serde(rename = "expMonth", default)]
    pub exp_month: String,
    /// The card expiration year for use with PAN based transactions.
    #[serde(rename = "expYear", default)]
    pub exp_year: String,
    /// The card CVV for use with PAN based transactions.
    #[serde(rename = "cvv", default)]
    pub cvv: String,
    /// The cardholder address for use with address verification.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The cardholder postal code for use with address verification.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The cardholder country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// That the payment entry method is a manual keyed transaction. If this is true, no other
/// payment method will be accepted.
    #[serde(rename = "manualEntry", default)]
    pub manual_entry: bool,
    /// The key serial number used for DUKPT encryption.
    #[serde(rename = "ksn", default)]
    pub ksn: String,
    /// The encrypted pin block.
    #[serde(rename = "pinBlock", default)]
    pub pin_block: String,
    /// Designates categories of cards: credit, debit, EBT.
    #[serde(rename = "cardType", default)]
    pub card_type: CardType,
    /// Designates brands of payment methods: Visa, Discover, etc.
    #[serde(rename = "paymentType", default)]
    pub payment_type: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,
    /// The method by which the payment card was entered (MSR, CHIP, KEYED, etc.).
    #[serde(rename = "entryMethod", default)]
    pub entry_method: String,
    /// Customer with which the new token should be associated.
    #[serde(rename = "customer")]
    pub customer: Option<Customer>,
    /// That this transaction should be treated as a recurring transaction.
    #[serde(rename = "recurring", default)]
    pub recurring: bool,
    /// That this transaction and any using this token should be treated as a subscription
/// recurring transaction.
    #[serde(rename = "subscription", default)]
    pub subscription: bool,
    /// That this transaction will include a card metadata lookup.
    #[serde(rename = "cardMetadataLookup", default)]
    pub card_metadata_lookup: bool,
    /// The type of account (checking, savings, etc) for an ACH payment method.
    #[serde(rename = "accountType", default)]
    pub account_type: String,
    /// The type of account holder (personal, business, etc) for an ACH payment method.
    #[serde(rename = "accountHolderType", default)]
    pub account_holder_type: String,
    /// The bank name for an ACH payment method.
    #[serde(rename = "bankName", default)]
    pub bank_name: String,

}

/// The response to an enroll request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnrollResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// That the transaction was approved.
    #[serde(rename = "approved")]
    pub approved: bool,
    /// The auth code from the payment network.
    #[serde(rename = "authCode", default)]
    pub auth_code: String,
    /// The code returned by the terminal or the card issuer to indicate the disposition of the
/// message.
    #[serde(rename = "authResponseCode", default)]
    pub auth_response_code: String,
    /// The ID assigned to the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The ID assigned to the batch.
    #[serde(rename = "batchId", default)]
    pub batch_id: String,
    /// The transaction reference string assigned to the transaction request. If no
/// transaction ref was assiged on the request, then the gateway will randomly generate
/// one.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// The type of transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The hash of the last tick block.
    #[serde(rename = "tickBlock")]
    pub tick_block: String,
    /// That the transaction was processed on the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// The ECC signature of the response. Can be used to ensure that it was signed by the
/// terminal and detect man-in-the middle attacks.
    #[serde(rename = "sig", default)]
    pub sig: String,
    /// The payment token, if the payment was enrolled in the vault.
    #[serde(rename = "token", default)]
    pub token: String,
    /// The entry method for the transaction (CHIP, MSR, KEYED, etc).
    #[serde(rename = "entryMethod", default)]
    pub entry_method: String,
    /// The card brand (VISA, MC, AMEX, DEBIT, etc).
    #[serde(rename = "paymentType", default)]
    pub payment_type: String,
    /// Provides network level detail on how a transaction was routed, especially for debit
/// transactions.
    #[serde(rename = "network", default)]
    pub network: String,
    /// Identifies the card association based on bin number. Used primarily used to indicate
/// the major logo on a card, even when debit transactions are routed on a different
/// network.
    #[serde(rename = "logo", default)]
    pub logo: String,
    /// The masked primary account number.
    #[serde(rename = "maskedPan", default)]
    pub masked_pan: String,
    /// The BlockChyp public key if the user presented a BlockChyp payment card.
    #[serde(rename = "publicKey", default)]
    pub public_key: String,
    /// That the transaction did something that would put the system in PCI scope.
    #[serde(rename = "ScopeAlert", default)]
    pub scope_alert: bool,
    /// The cardholder name.
    #[serde(rename = "cardHolder", default)]
    pub card_holder: String,
    /// The card expiration month in MM format.
    #[serde(rename = "expMonth", default)]
    pub exp_month: String,
    /// The card expiration year in YY format.
    #[serde(rename = "expYear", default)]
    pub exp_year: String,
    /// The card postal code.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The card address.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The card country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// Address verification results if address information was submitted.
    #[serde(rename = "avsResponse")]
    pub avs_response: AVSResponse,
    /// The CVV verification result if CVV was submitted.
    #[serde(rename = "cvvResponse", default)]
    pub cvv_response: String,
    /// Suggested receipt fields.
    #[serde(rename = "receiptSuggestions")]
    pub receipt_suggestions: ReceiptSuggestions,
    /// Customer data, if any. Preserved for reverse compatibility.
    #[serde(rename = "customer")]
    pub customer: Option<Customer>,
    /// Customer data, if any.
    #[serde(rename = "customers")]
    pub customers: Option<Vec<Customer>>,
    /// The hex encoded signature data.
    #[serde(rename = "sigFile", default)]
    pub sig_file: String,
    /// Details about a payment card derived from its BIN/IIN.
    #[serde(rename = "cardMetadata", default)]
    pub card_metadata: Option<CardMetadata>,
    /// The type of account (checking, savings, etc) for an ACH payment method.
    #[serde(rename = "accountType", default)]
    pub account_type: String,
    /// The type of account holder (personal, business, etc) for an ACH payment method.
    #[serde(rename = "accountHolderType", default)]
    pub account_holder_type: String,
    /// The bank name for an ACH payment method.
    #[serde(rename = "bankName", default)]
    pub bank_name: String,
    /// The token hash (generated with a static salt, Merchant ID, Registration Date and PAN).
    #[serde(rename = "tokenHash", default)]
    pub token_hash: String,
    /// The first 8 digits of the card aka the BIN.
    #[serde(rename = "bin", default)]
    pub bin: String,

}

/// The information needed to enroll a new payment method in the token vault.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ClearTerminalRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,

}

/// The information needed to activate or recharge a gift card.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GiftActivateRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The transaction currency code.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The requested amount.
    #[serde(rename = "amount")]
    pub amount: String,
    /// That the request is tax exempt. Only required for tax exempt level 2 processing.
    #[serde(rename = "taxExempt")]
    pub tax_exempt: bool,
    /// A flag to add a surcharge to the transaction to cover credit card fees, if permitted.
    #[serde(rename = "surcharge")]
    pub surcharge: bool,
    /// A flag that applies a discount to negate the surcharge for debit transactions or other
/// surcharge ineligible payment methods.
    #[serde(rename = "cashDiscount")]
    pub cash_discount: bool,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,

}

/// The response to a gift activate request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GiftActivateResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The ID assigned to the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The ID assigned to the batch.
    #[serde(rename = "batchId", default)]
    pub batch_id: String,
    /// The transaction reference string assigned to the transaction request. If no
/// transaction ref was assiged on the request, then the gateway will randomly generate
/// one.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// The type of transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The hash of the last tick block.
    #[serde(rename = "tickBlock")]
    pub tick_block: String,
    /// That the transaction was processed on the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// The ECC signature of the response. Can be used to ensure that it was signed by the
/// terminal and detect man-in-the middle attacks.
    #[serde(rename = "sig", default)]
    pub sig: String,
    /// That the card was activated.
    #[serde(rename = "approved")]
    pub approved: bool,
    /// The amount of the transaction.
    #[serde(rename = "amount")]
    pub amount: String,
    /// The current balance of the gift card.
    #[serde(rename = "currentBalance")]
    pub current_balance: String,
    /// The currency code used for the transaction.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The public key of the activated card.
    #[serde(rename = "publicKey")]
    pub public_key: String,
    /// The masked card identifier.
    #[serde(rename = "maskedPan", default)]
    pub masked_pan: String,

}

/// The information needed to manually close a credit card batch.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CloseBatchRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// Optional batch id.
    #[serde(rename = "batchId")]
    pub batch_id: String,

}

/// The response to a close batch request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CloseBatchResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The ID assigned to the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The ID assigned to the batch.
    #[serde(rename = "batchId", default)]
    pub batch_id: String,
    /// The transaction reference string assigned to the transaction request. If no
/// transaction ref was assiged on the request, then the gateway will randomly generate
/// one.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// The type of transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The hash of the last tick block.
    #[serde(rename = "tickBlock")]
    pub tick_block: String,
    /// That the transaction was processed on the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// The ECC signature of the response. Can be used to ensure that it was signed by the
/// terminal and detect man-in-the middle attacks.
    #[serde(rename = "sig", default)]
    pub sig: String,
    /// A collection of batches closed during the batch close operation.
    #[serde(rename = "batches")]
    pub batches: Option<Vec<BatchSummary>>,

}

/// The fields needed for custom Terms and Conditions prompts.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TermsAndConditionsRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The ID of the previous transaction being referenced.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// A location on the filesystem which a customer signature should be written to.
    #[serde(rename = "sigFile", default)]
    pub sig_file: String,
    /// The image format to be used for returning signatures.
    #[serde(rename = "sigFormat", default)]
    pub sig_format: SignatureFormat,
    /// The width that the signature image should be scaled to, preserving the aspect ratio. If
/// not provided, the signature is returned in the terminal's max resolution.
    #[serde(rename = "sigWidth", default)]
    pub sig_width: i32,
    /// Whether or not signature prompt should be skipped on the terminal. The terminal will
/// indicate whether or not a signature is required by the card in the receipt suggestions
/// response.
    #[serde(rename = "disableSignature", default)]
    pub disable_signature: bool,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,
    /// An alias for a Terms and Conditions template configured in the BlockChyp dashboard.
    #[serde(rename = "tcAlias")]
    pub tc_alias: String,
    /// The name of the Terms and Conditions the user is accepting.
    #[serde(rename = "tcName")]
    pub tc_name: String,
    /// The content of the terms and conditions that will be presented to the user.
    #[serde(rename = "tcContent")]
    pub tc_content: String,
    /// A hash of the terms and conditions content that can be used for caching.
    #[serde(rename = "contentHash")]
    pub content_hash: String,
    /// That a signature should be requested.
    #[serde(rename = "sigRequired")]
    pub sig_required: bool,

}

/// A signature capture response for Terms and Conditions.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TermsAndConditionsResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The ID assigned to the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The ID assigned to the batch.
    #[serde(rename = "batchId", default)]
    pub batch_id: String,
    /// The transaction reference string assigned to the transaction request. If no
/// transaction ref was assiged on the request, then the gateway will randomly generate
/// one.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// The type of transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The hash of the last tick block.
    #[serde(rename = "tickBlock")]
    pub tick_block: String,
    /// That the transaction was processed on the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// The ECC signature of the response. Can be used to ensure that it was signed by the
/// terminal and detect man-in-the middle attacks.
    #[serde(rename = "sig", default)]
    pub sig: String,
    /// The hex encoded signature data.
    #[serde(rename = "sigFile", default)]
    pub sig_file: String,

}

/// The response to an authorization request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthorizationResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// That the transaction was approved.
    #[serde(rename = "approved")]
    pub approved: bool,
    /// The auth code from the payment network.
    #[serde(rename = "authCode", default)]
    pub auth_code: String,
    /// The code returned by the terminal or the card issuer to indicate the disposition of the
/// message.
    #[serde(rename = "authResponseCode", default)]
    pub auth_response_code: String,
    /// The ID assigned to the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The ID assigned to the batch.
    #[serde(rename = "batchId", default)]
    pub batch_id: String,
    /// The transaction reference string assigned to the transaction request. If no
/// transaction ref was assiged on the request, then the gateway will randomly generate
/// one.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// The type of transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The hash of the last tick block.
    #[serde(rename = "tickBlock")]
    pub tick_block: String,
    /// That the transaction was processed on the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// The ECC signature of the response. Can be used to ensure that it was signed by the
/// terminal and detect man-in-the middle attacks.
    #[serde(rename = "sig", default)]
    pub sig: String,
    /// Whether or not the transaction was approved for a partial amount.
    #[serde(rename = "partialAuth")]
    pub partial_auth: bool,
    /// Whether or not an alternate currency was used.
    #[serde(rename = "altCurrency")]
    pub alt_currency: bool,
    /// Whether or not a request was settled on an FSA card.
    #[serde(rename = "fsaAuth")]
    pub fsa_auth: bool,
    /// The currency code used for the transaction.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The requested amount.
    #[serde(rename = "requestedAmount")]
    pub requested_amount: String,
    /// The authorized amount. May not match the requested amount in the event of a partial
/// auth.
    #[serde(rename = "authorizedAmount")]
    pub authorized_amount: String,
    /// The remaining balance on the payment method.
    #[serde(rename = "remainingBalance")]
    pub remaining_balance: String,
    /// The tip amount.
    #[serde(rename = "tipAmount")]
    pub tip_amount: String,
    /// The tax amount.
    #[serde(rename = "taxAmount")]
    pub tax_amount: String,
    /// The cash back amount the customer requested during the transaction.
    #[serde(rename = "requestedCashBackAmount")]
    pub requested_cash_back_amount: String,
    /// The amount of cash back authorized by the gateway. This amount will be the entire amount
/// requested, or zero.
    #[serde(rename = "authorizedCashBackAmount")]
    pub authorized_cash_back_amount: String,
    /// That the transaction has met the standard criteria for confirmation on the network.
/// (For example, 6 confirmations for level one bitcoin.)
    #[serde(rename = "confirmed")]
    pub confirmed: bool,
    /// The amount submitted to the blockchain.
    #[serde(rename = "cryptoAuthorizedAmount")]
    pub crypto_authorized_amount: String,
    /// The network level fee assessed for the transaction denominated in cryptocurrency.
/// This fee goes to channel operators and crypto miners, not BlockChyp.
    #[serde(rename = "cryptoNetworkFee")]
    pub crypto_network_fee: String,
    /// The three letter cryptocurrency code used for the transactions.
    #[serde(rename = "cryptocurrency")]
    pub cryptocurrency: String,
    /// Whether or not the transaction was processed on the level one or level two network.
    #[serde(rename = "cryptoNetwork")]
    pub crypto_network: String,
    /// The address on the crypto network the transaction was sent to.
    #[serde(rename = "cryptoReceiveAddress")]
    pub crypto_receive_address: String,
    /// Hash or other identifier that identifies the block on the cryptocurrency network, if
/// available or relevant.
    #[serde(rename = "cryptoBlock")]
    pub crypto_block: String,
    /// Hash or other transaction identifier that identifies the transaction on the
/// cryptocurrency network, if available or relevant.
    #[serde(rename = "cryptoTransactionId")]
    pub crypto_transaction_id: String,
    /// The payment request URI used for the transaction, if available.
    #[serde(rename = "cryptoPaymentRequest")]
    pub crypto_payment_request: String,
    /// Used for additional status information related to crypto transactions.
    #[serde(rename = "cryptoStatus")]
    pub crypto_status: String,
    /// The payment token, if the payment was enrolled in the vault.
    #[serde(rename = "token", default)]
    pub token: String,
    /// The entry method for the transaction (CHIP, MSR, KEYED, etc).
    #[serde(rename = "entryMethod", default)]
    pub entry_method: String,
    /// The card brand (VISA, MC, AMEX, DEBIT, etc).
    #[serde(rename = "paymentType", default)]
    pub payment_type: String,
    /// Provides network level detail on how a transaction was routed, especially for debit
/// transactions.
    #[serde(rename = "network", default)]
    pub network: String,
    /// Identifies the card association based on bin number. Used primarily used to indicate
/// the major logo on a card, even when debit transactions are routed on a different
/// network.
    #[serde(rename = "logo", default)]
    pub logo: String,
    /// The masked primary account number.
    #[serde(rename = "maskedPan", default)]
    pub masked_pan: String,
    /// The BlockChyp public key if the user presented a BlockChyp payment card.
    #[serde(rename = "publicKey", default)]
    pub public_key: String,
    /// That the transaction did something that would put the system in PCI scope.
    #[serde(rename = "ScopeAlert", default)]
    pub scope_alert: bool,
    /// The cardholder name.
    #[serde(rename = "cardHolder", default)]
    pub card_holder: String,
    /// The card expiration month in MM format.
    #[serde(rename = "expMonth", default)]
    pub exp_month: String,
    /// The card expiration year in YY format.
    #[serde(rename = "expYear", default)]
    pub exp_year: String,
    /// The card postal code.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The card address.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The card country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// Address verification results if address information was submitted.
    #[serde(rename = "avsResponse")]
    pub avs_response: AVSResponse,
    /// The CVV verification result if CVV was submitted.
    #[serde(rename = "cvvResponse", default)]
    pub cvv_response: String,
    /// Suggested receipt fields.
    #[serde(rename = "receiptSuggestions")]
    pub receipt_suggestions: ReceiptSuggestions,
    /// Customer data, if any. Preserved for reverse compatibility.
    #[serde(rename = "customer")]
    pub customer: Option<Customer>,
    /// Customer data, if any.
    #[serde(rename = "customers")]
    pub customers: Option<Vec<Customer>>,
    /// The hex encoded signature data.
    #[serde(rename = "sigFile", default)]
    pub sig_file: String,
    /// Card BIN ranges can be whitelisted so that they are read instead of being processed
/// directly. This is useful for integration with legacy gift card systems.
    #[serde(rename = "whiteListedCard")]
    pub white_listed_card: Option<WhiteListedCard>,
    /// That the transaction was flagged for store and forward due to network problems.
    #[serde(rename = "storeAndForward")]
    pub store_and_forward: bool,
    /// The current status of a transaction.
    #[serde(rename = "status")]
    pub status: String,
    /// Details about a payment card derived from its BIN/IIN.
    #[serde(rename = "cardMetadata", default)]
    pub card_metadata: Option<CardMetadata>,

}

/// Models the request for updated information about a transaction.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionStatusRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The BlockChyp assigned transaction id.
    #[serde(rename = "transactionId", default)]
    pub transaction_id: String,

}

/// Models the request for updated information about a payment link.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinkStatusRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The link code assigned to the payment link.
    #[serde(rename = "linkCode")]
    pub link_code: String,

}

/// Models the status of a payment link.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinkStatusResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The code used to retrieve the payment link.
    #[serde(rename = "linkCode")]
    pub link_code: String,
    /// The BlockChyp merchant id associated with a payment link.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    /// The customer id associated with a payment link.
    #[serde(rename = "customerId", default)]
    pub customer_id: String,
    /// The user's internal reference for any transaction that may occur.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// The user's internal reference for an order.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// That the order is tax exempt.
    #[serde(rename = "taxExempt", default)]
    pub tax_exempt: bool,
    /// That the amount to collect via the payment link.
    #[serde(rename = "amount", default)]
    pub amount: String,
    /// The sales tax to be collected via the payment link.
    #[serde(rename = "taxAmount", default)]
    pub tax_amount: String,
    /// Subject for email notifications.
    #[serde(rename = "subject", default)]
    pub subject: String,
    /// Id of the most recent transaction associated with the link.
    #[serde(rename = "transactionId", default)]
    pub transaction_id: String,
    /// Description associated with the payment link.
    #[serde(rename = "description", default)]
    pub description: String,
    /// Date and time the link will expire.
    #[serde(rename = "expiration", default)]
    pub expiration: Option<DateTime<Utc>>,
    /// Date and time the link was created.
    #[serde(rename = "dateCreated", default)]
    pub date_created: DateTime<Utc>,
    /// Line item level data if provided.
    #[serde(rename = "transactionDetails", default)]
    pub transaction_details: Option<TransactionDisplayTransaction>,
    /// The current status of the payment link.
    #[serde(rename = "status", default)]
    pub status: String,
    /// Alias for any terms and conditions language associated with the link.
    #[serde(rename = "tcAlias", default)]
    pub tc_alias: String,
    /// Name of any terms and conditions agreements associated with the payment link.
    #[serde(rename = "tcName", default)]
    pub tc_name: String,
    /// Full text of any terms and conditions language associated with the agreement.
    #[serde(rename = "tcContent", default)]
    pub tc_content: String,
    /// That the link is intended for internal use by the merchant.
    #[serde(rename = "cashierFacing", default)]
    pub cashier_facing: bool,
    /// That the payment method should be enrolled in the token vault.
    #[serde(rename = "enroll", default)]
    pub enroll: bool,
    /// That the link should only be used for enrollment in the token vault without any
/// underlying payment transaction.
    #[serde(rename = "enrollOnly", default)]
    pub enroll_only: bool,
    /// Returns details about the last transaction status.
    #[serde(rename = "lastTransaction", default)]
    pub last_transaction: Option<AuthorizationResponse>,
    /// Returns a list of transactions associated with the link, including any declines.
    #[serde(rename = "transactionHistory", default)]
    pub transaction_history: Option<Vec<AuthorizationResponse>>,

}

/// Models the status of a transaction.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionStatus {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// That the transaction was approved.
    #[serde(rename = "approved")]
    pub approved: bool,
    /// The auth code from the payment network.
    #[serde(rename = "authCode", default)]
    pub auth_code: String,
    /// The code returned by the terminal or the card issuer to indicate the disposition of the
/// message.
    #[serde(rename = "authResponseCode", default)]
    pub auth_response_code: String,
    /// The ID assigned to the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The ID assigned to the batch.
    #[serde(rename = "batchId", default)]
    pub batch_id: String,
    /// The transaction reference string assigned to the transaction request. If no
/// transaction ref was assiged on the request, then the gateway will randomly generate
/// one.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// The type of transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The hash of the last tick block.
    #[serde(rename = "tickBlock")]
    pub tick_block: String,
    /// That the transaction was processed on the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// The ECC signature of the response. Can be used to ensure that it was signed by the
/// terminal and detect man-in-the middle attacks.
    #[serde(rename = "sig", default)]
    pub sig: String,
    /// Whether or not the transaction was approved for a partial amount.
    #[serde(rename = "partialAuth")]
    pub partial_auth: bool,
    /// Whether or not an alternate currency was used.
    #[serde(rename = "altCurrency")]
    pub alt_currency: bool,
    /// Whether or not a request was settled on an FSA card.
    #[serde(rename = "fsaAuth")]
    pub fsa_auth: bool,
    /// The currency code used for the transaction.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The requested amount.
    #[serde(rename = "requestedAmount")]
    pub requested_amount: String,
    /// The authorized amount. May not match the requested amount in the event of a partial
/// auth.
    #[serde(rename = "authorizedAmount")]
    pub authorized_amount: String,
    /// The remaining balance on the payment method.
    #[serde(rename = "remainingBalance")]
    pub remaining_balance: String,
    /// The tip amount.
    #[serde(rename = "tipAmount")]
    pub tip_amount: String,
    /// The tax amount.
    #[serde(rename = "taxAmount")]
    pub tax_amount: String,
    /// The cash back amount the customer requested during the transaction.
    #[serde(rename = "requestedCashBackAmount")]
    pub requested_cash_back_amount: String,
    /// The amount of cash back authorized by the gateway. This amount will be the entire amount
/// requested, or zero.
    #[serde(rename = "authorizedCashBackAmount")]
    pub authorized_cash_back_amount: String,
    /// The payment token, if the payment was enrolled in the vault.
    #[serde(rename = "token", default)]
    pub token: String,
    /// The entry method for the transaction (CHIP, MSR, KEYED, etc).
    #[serde(rename = "entryMethod", default)]
    pub entry_method: String,
    /// The card brand (VISA, MC, AMEX, DEBIT, etc).
    #[serde(rename = "paymentType", default)]
    pub payment_type: String,
    /// Provides network level detail on how a transaction was routed, especially for debit
/// transactions.
    #[serde(rename = "network", default)]
    pub network: String,
    /// Identifies the card association based on bin number. Used primarily used to indicate
/// the major logo on a card, even when debit transactions are routed on a different
/// network.
    #[serde(rename = "logo", default)]
    pub logo: String,
    /// The masked primary account number.
    #[serde(rename = "maskedPan", default)]
    pub masked_pan: String,
    /// The BlockChyp public key if the user presented a BlockChyp payment card.
    #[serde(rename = "publicKey", default)]
    pub public_key: String,
    /// That the transaction did something that would put the system in PCI scope.
    #[serde(rename = "ScopeAlert", default)]
    pub scope_alert: bool,
    /// The cardholder name.
    #[serde(rename = "cardHolder", default)]
    pub card_holder: String,
    /// The card expiration month in MM format.
    #[serde(rename = "expMonth", default)]
    pub exp_month: String,
    /// The card expiration year in YY format.
    #[serde(rename = "expYear", default)]
    pub exp_year: String,
    /// The card postal code.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    /// The card address.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The card country.
    #[serde(rename = "country", default)]
    pub country: String,
    /// Address verification results if address information was submitted.
    #[serde(rename = "avsResponse")]
    pub avs_response: AVSResponse,
    /// The CVV verification result if CVV was submitted.
    #[serde(rename = "cvvResponse", default)]
    pub cvv_response: String,
    /// Suggested receipt fields.
    #[serde(rename = "receiptSuggestions")]
    pub receipt_suggestions: ReceiptSuggestions,
    /// Customer data, if any. Preserved for reverse compatibility.
    #[serde(rename = "customer")]
    pub customer: Option<Customer>,
    /// Customer data, if any.
    #[serde(rename = "customers")]
    pub customers: Option<Vec<Customer>>,
    /// The hex encoded signature data.
    #[serde(rename = "sigFile", default)]
    pub sig_file: String,
    /// That the transaction was flagged for store and forward due to network problems.
    #[serde(rename = "storeAndForward")]
    pub store_and_forward: bool,

}

/// An item level discount for transaction display. Discounts never combine.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionDisplayDiscount {
    /// The discount description.
    #[serde(rename = "description")]
    pub description: String,
    /// The amount of the discount.
    #[serde(rename = "amount")]
    pub amount: String,

}

/// An item category in a transaction display. Groups combine if their descriptions
/// match. Calculated subtotal amounts are rounded to two decimal places of precision.
/// Quantity is a floating point number that is not rounded at all.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionDisplayItem {
    /// A unique value identifying the item. This is not required, but recommended since it is
/// required to update or delete line items.
    #[serde(rename = "id")]
    pub id: String,
    /// A description of the line item.
    #[serde(rename = "description")]
    pub description: String,
    /// The price of the line item.
    #[serde(rename = "price")]
    pub price: String,
    /// The quantity of the line item.
    #[serde(rename = "quantity")]
    pub quantity: f64,
    /// An item category in a transaction display. Groups combine if their descriptions
/// match. Calculated subtotal amounts are rounded to two decimal places of precision.
/// Quantity is a floating point number that is not rounded at all.
    #[serde(rename = "extended")]
    pub extended: String,
    /// An alphanumeric code for units of measurement as used in international trade.
    #[serde(rename = "unitCode")]
    pub unit_code: String,
    /// An international description code of the item.
    #[serde(rename = "commodityCode")]
    pub commodity_code: String,
    /// A merchant-defined description code of the item.
    #[serde(rename = "productCode")]
    pub product_code: String,
    /// Are displayed under their corresponding item.
    #[serde(rename = "discounts")]
    pub discounts: Option<Vec<TransactionDisplayDiscount>>,
    /// The amount of any value added taxes which apply to the item.
    #[serde(rename = "taxAmount", default)]
    pub tax_amount: String,
    /// The tax rate as a percentage. Example: '8.5' for 8.5% tax rate.
    #[serde(rename = "taxRate", default)]
    pub tax_rate: String,
    /// How tax was applied to discounted items. '0' = no discount, '1' = tax calculated after
/// discount, '2' = taxcalculated before discount.
    #[serde(rename = "discountCode", default)]
    pub discount_code: String,

}

/// The items to display on a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionDisplayTransaction {
    /// The subtotal to display.
    #[serde(rename = "subtotal")]
    pub subtotal: String,
    /// The tax to display.
    #[serde(rename = "tax")]
    pub tax: String,
    /// The total to display.
    #[serde(rename = "total")]
    pub total: String,
    /// An item to display. Can be overwritten or appended, based on the request type.
    #[serde(rename = "items")]
    pub items: Option<Vec<TransactionDisplayItem>>,

}

/// Used to start or update a transaction line item display on a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionDisplayRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,
    /// Transaction to display on the terminal.
    #[serde(rename = "transaction")]
    pub transaction: Option<TransactionDisplayTransaction>,

}

/// The response to a basic API health check. If the security context permits it, the
/// response may also include the public key of the current merchant.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HeartbeatResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The timestamp of the heartbeat.
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    /// The public key of the clockchain. This is blockchain stuff that you don't really need to
/// worry about. It is a base 58 encoded and compressed eliptic curve public key. For the
/// production clockchain, this will always be:
/// '3cuhsckVUd9HzMjbdUSW17aY5kCcm1d6YAphJMUwmtXRj7WLyU'.
    #[serde(rename = "clockchain")]
    pub clockchain: String,
    /// The hash of the last tick block.
    #[serde(rename = "latestTick")]
    pub latest_tick: String,
    /// The public key for the merchant's blockchain.
    #[serde(rename = "merchantPublicKey")]
    pub merchant_public_key: String,

}

/// A request for the status of a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalStatusRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,

}

/// The current status of a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalStatusResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// That the terminal is idle.
    #[serde(rename = "idle")]
    pub idle: bool,
    /// Whether or not a card is currently in the card slot.
    #[serde(rename = "cardInSlot")]
    pub card_in_slot: bool,
    /// The operation that the terminal is performing.
    #[serde(rename = "status")]
    pub status: String,
    /// The transaction reference for an ongoing transaction, if one was specified at request
/// time.
    #[serde(rename = "transactionRef")]
    pub transaction_ref: String,
    /// The timestamp of the last status change.
    #[serde(rename = "since")]
    pub since: DateTime<Utc>,

}

/// Creates a payment link.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinkRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The transaction currency code.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The requested amount.
    #[serde(rename = "amount")]
    pub amount: String,
    /// That the request is tax exempt. Only required for tax exempt level 2 processing.
    #[serde(rename = "taxExempt")]
    pub tax_exempt: bool,
    /// A flag to add a surcharge to the transaction to cover credit card fees, if permitted.
    #[serde(rename = "surcharge")]
    pub surcharge: bool,
    /// A flag that applies a discount to negate the surcharge for debit transactions or other
/// surcharge ineligible payment methods.
    #[serde(rename = "cashDiscount")]
    pub cash_discount: bool,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,
    /// Automatically send the link via an email.
    #[serde(rename = "autoSend")]
    pub auto_send: bool,
    /// That the payment method should be added to the token vault alongside the
/// authorization.
    #[serde(rename = "enroll", default)]
    pub enroll: bool,
    /// That the link should be used to enroll a token only. Can only be used in cashier mode.
    #[serde(rename = "enrollOnly", default)]
    pub enroll_only: bool,
    /// That the QR Code binary should be returned.
    #[serde(rename = "qrcodeBinary", default)]
    pub qrcode_binary: bool,
    /// Determines the size of the qr code to be returned.
    #[serde(rename = "qrcodeSize", default)]
    pub qrcode_size: i32,
    /// Number of days until the payment link expires.
    #[serde(rename = "daysToExpiration", default)]
    pub days_to_expiration: i32,
    /// Flags the payment link as cashier facing.
    #[serde(rename = "cashier")]
    pub cashier: bool,
    /// Description explaining the transaction for display to the user.
    #[serde(rename = "description")]
    pub description: String,
    /// Subject of the payment email.
    #[serde(rename = "subject")]
    pub subject: String,
    /// Transaction details for display on the payment email.
    #[serde(rename = "transaction")]
    pub transaction: Option<TransactionDisplayTransaction>,
    /// Customer information.
    #[serde(rename = "customer")]
    pub customer: Customer,
    /// Optional callback url to which transaction responses for this link will be posted.
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    /// An alias for a Terms and Conditions template configured in the BlockChyp dashboard.
    #[serde(rename = "tcAlias")]
    pub tc_alias: String,
    /// The name of the Terms and Conditions the user is accepting.
    #[serde(rename = "tcName")]
    pub tc_name: String,
    /// The content of the terms and conditions that will be presented to the user.
    #[serde(rename = "tcContent")]
    pub tc_content: String,
    /// That the transaction should be a cryptocurrency transaction. Value should be a crypto
/// currency code (ETH, BTC) or ANY to prompt the user to choose from supported
/// cryptocurrencies.
    #[serde(rename = "cryptocurrency")]
    pub cryptocurrency: Option<String>,
    /// An optional parameter that can be used to force a crypto transaction onto a level one or
/// level two network. Valid values are L1 and L2. Defaults to L1.
    #[serde(rename = "cryptoNetwork")]
    pub crypto_network: Option<String>,
    /// Can be used to specify a specific receive address for a crypto transaction. Disabled by
/// default. This should only be used by sophisticated users with access to properly
/// configured hot wallets.
    #[serde(rename = "cryptoReceiveAddress")]
    pub crypto_receive_address: Option<String>,
    /// Can optionally add a label to the payment request if the target cryptocurrency
/// supports labels. Defaults to the merchant's DBA Name.
    #[serde(rename = "paymentRequestLabel")]
    pub payment_request_label: Option<String>,
    /// Can optionally add a message to the payment request if the target cryptocurrency
/// supports labels. Defaults to empty.
    #[serde(rename = "paymentRequestMessage")]
    pub payment_request_message: Option<String>,

}

/// Creates a payment link.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinkResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The payment link code.
    #[serde(rename = "linkCode")]
    pub link_code: String,
    /// The url for the payment link.
    #[serde(rename = "url")]
    pub url: String,
    /// The url for a QR Code associated with this link.
    #[serde(rename = "qrcodeUrl")]
    pub qrcode_url: String,
    /// The hex encoded binary for the QR Code, if requested. Encoded in PNG format.
    #[serde(rename = "qrcodeBinary")]
    pub qrcode_binary: String,
    /// The customer id created or used for the payment.
    #[serde(rename = "customerId")]
    pub customer_id: String,

}

/// Cancels a pending payment link. Payment links that have already been used cannot be
/// canceled and the request will be rejected.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CancelPaymentLinkRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The payment link code to cancel.
    #[serde(rename = "linkCode")]
    pub link_code: String,

}

/// Success or failure of a payment link cancellation.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CancelPaymentLinkResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,

}

/// Resends a pending payment link. Payment links that have already been used or cancelled
/// cannot be resent and the request will be rejected.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ResendPaymentLinkRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The payment link code to cancel.
    #[serde(rename = "linkCode")]
    pub link_code: String,

}

/// Success or failure of a payment link resend operation.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ResendPaymentLinkResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,

}

/// Computes the cash discount for a cash discount if enabled.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CashDiscountRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The transaction currency code.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The requested amount.
    #[serde(rename = "amount")]
    pub amount: String,
    /// That the request is tax exempt. Only required for tax exempt level 2 processing.
    #[serde(rename = "taxExempt")]
    pub tax_exempt: bool,
    /// A flag to add a surcharge to the transaction to cover credit card fees, if permitted.
    #[serde(rename = "surcharge")]
    pub surcharge: bool,
    /// A flag that applies a discount to negate the surcharge for debit transactions or other
/// surcharge ineligible payment methods.
    #[serde(rename = "cashDiscount")]
    pub cash_discount: bool,
    /// How partial pennies should be rounded for calculated values like surcharges.
/// Rounding up is the default behavior.
    #[serde(rename = "roundingMode")]
    pub rounding_mode: Option<RoundingMode>,

}

/// Models the results of a cash discount calculation.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CashDiscountResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The transaction currency code.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The new calculated total amount.
    #[serde(rename = "amount")]
    pub amount: String,
    /// That the request is tax exempt. Only required for tax exempt level 2 processing.
    #[serde(rename = "taxExempt")]
    pub tax_exempt: bool,
    /// The normal surcharge for a transaction. Will only be returned if an offsetting cash
/// discount is also returned.
    #[serde(rename = "surcharge")]
    pub surcharge: String,
    /// The cash discount. Will not be returned in surcharge only mode.
    #[serde(rename = "cashDiscount")]
    pub cash_discount: String,

}

/// Models a batch history request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionHistoryRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// Optional search query. Will match amount, last 4 and customer name. batchId and
/// terminalName are not supported with this option.
    #[serde(rename = "query")]
    pub query: String,
    /// Optional batch id.
    #[serde(rename = "batchId")]
    pub batch_id: String,
    /// Optional terminal name.
    #[serde(rename = "terminalName")]
    pub terminal_name: String,
    /// Optional start date filter for batch history.
    #[serde(rename = "startDate")]
    pub start_date: DateTime<Utc>,
    /// Optional end date filter for batch history.
    #[serde(rename = "endDate")]
    pub end_date: DateTime<Utc>,
    /// Max results to be returned by this request.
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    /// Starting index for results to be returned.
    #[serde(rename = "startIndex")]
    pub start_index: i32,

}

/// Models response to a batch history request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionHistoryResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// That the response came from the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Batch identifier if filtered by batch.
    #[serde(rename = "batchId")]
    pub batch_id: String,
    /// Terminal name if filtered by terminal.
    #[serde(rename = "terminalName")]
    pub terminal_name: String,
    /// Start date if filtered by start date.
    #[serde(rename = "startDate")]
    pub start_date: DateTime<Utc>,
    /// End date if filtered by end date.
    #[serde(rename = "endDate")]
    pub end_date: DateTime<Utc>,
    /// Max results from the original request echoed back. Defaults to the system max of 250.
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    /// Starting index from the original request echoed back.
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// Total number of results accessible through paging.
    #[serde(rename = "totalResultCount")]
    pub total_result_count: i32,
    /// Matching transaction history.
    #[serde(rename = "transactions")]
    pub transactions: Option<Vec<AuthorizationResponse>>,

}

/// Models a batch history request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BatchHistoryRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// Optional start date filter for batch history.
    #[serde(rename = "startDate")]
    pub start_date: DateTime<Utc>,
    /// Optional end date filter for batch history.
    #[serde(rename = "endDate")]
    pub end_date: DateTime<Utc>,
    /// Max results to be returned by this request. Defaults to the system max of 250.
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    /// Starting index for results to be returned.
    #[serde(rename = "startIndex")]
    pub start_index: i32,

}

/// Models response to a batch history request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BatchHistoryResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// That the response came from the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Start date if filtered by start date.
    #[serde(rename = "startDate")]
    pub start_date: DateTime<Utc>,
    /// End date if filtered by end date.
    #[serde(rename = "endDate")]
    pub end_date: DateTime<Utc>,
    /// Merchant's batch history in descending order.
    #[serde(rename = "batches")]
    pub batches: Option<Vec<BatchSummary>>,
    /// Max results from the original request echoed back.
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    /// Starting index from the original request echoed back.
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// Total number of results accessible through paging.
    #[serde(rename = "totalResultCount")]
    pub total_result_count: i32,

}

/// Models high level information about a single batch.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BatchSummary {
    /// Batch identifier.
    #[serde(rename = "batchId")]
    pub batch_id: String,
    /// Entry method for the batch, if any.
    #[serde(rename = "entryMethod")]
    pub entry_method: String,
    /// Merchant deposit account into which proceeds should be deposited.
    #[serde(rename = "destinationAccountId")]
    pub destination_account_id: String,
    /// The new captured amount.
    #[serde(rename = "capturedAmount")]
    pub captured_amount: String,
    /// The amount of preauths opened during the batch that have not been captured.
    #[serde(rename = "openPreauths")]
    pub open_preauths: String,
    /// The currency the batch was settled in.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// Flag indicating whether or not the batch is open.
    #[serde(rename = "open")]
    pub open: bool,
    /// Date and time of the first transaction for this batch.
    #[serde(rename = "openDate")]
    pub open_date: DateTime<Utc>,
    /// Date and time the batch was closed.
    #[serde(rename = "closeDate")]
    pub close_date: DateTime<Utc>,

}

/// Models a request for details about a single batch.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BatchDetailsRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// Id for the batch to be retrieved.
    #[serde(rename = "batchId")]
    pub batch_id: String,

}

/// Models a response for details about a single batch.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BatchDetailsResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// That the response came from the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Batch identifier.
    #[serde(rename = "batchId")]
    pub batch_id: String,
    /// Entry method for the batch, if any.
    #[serde(rename = "entryMethod")]
    pub entry_method: String,
    /// Merchant deposit account into which proceeds should be deposited.
    #[serde(rename = "destinationAccountId")]
    pub destination_account_id: String,
    /// The new captured amount.
    #[serde(rename = "capturedAmount")]
    pub captured_amount: String,
    /// Preauths from this batch still open.
    #[serde(rename = "openPreauths")]
    pub open_preauths: String,
    /// The total volume from this batch.
    #[serde(rename = "totalVolume")]
    pub total_volume: String,
    /// The total number of transactions in this batch.
    #[serde(rename = "transactionCount")]
    pub transaction_count: i32,
    /// The total volume of gift cards sold.
    #[serde(rename = "giftCardsSold")]
    pub gift_cards_sold: String,
    /// The total volume of gift cards transactions.
    #[serde(rename = "giftCardVolume")]
    pub gift_card_volume: String,
    /// The expected volume for this batch, usually captured volume less gift card volume.
    #[serde(rename = "expectedDeposit")]
    pub expected_deposit: String,
    /// Flag indicating whether or not the batch is open.
    #[serde(rename = "open")]
    pub open: bool,
    /// Date and time of the first transaction for this batch.
    #[serde(rename = "openDate")]
    pub open_date: DateTime<Utc>,
    /// Date and time the batch was closed.
    #[serde(rename = "closeDate")]
    pub close_date: DateTime<Utc>,
    /// Merchant's batch history in descending order.
    #[serde(rename = "volumeByTerminal")]
    pub volume_by_terminal: Option<Vec<TerminalVolume>>,
    /// The net volume for this batch, usually expected volume less daily fees volume.
    #[serde(rename = "netDeposit")]
    pub net_deposit: String,
    /// The daily fees for this batch
    #[serde(rename = "dailyFees")]
    pub daily_fees: String,

}

/// Models transaction volume for a single terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalVolume {
    /// The terminal name assigned during activation.
    #[serde(rename = "terminalName")]
    pub terminal_name: String,
    /// The manufacturer's serial number.
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    /// The terminal type.
    #[serde(rename = "terminalType")]
    pub terminal_type: String,
    /// The captured amount.
    #[serde(rename = "capturedAmount")]
    pub captured_amount: String,
    /// The number of transactions run on this terminal.
    #[serde(rename = "transactionCount")]
    pub transaction_count: i32,

}

/// Models basic information needed to create a test merchant.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AddTestMerchantRequest {
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The DBA name for the test merchant.
    #[serde(rename = "dbaName")]
    pub dba_name: String,
    /// The corporate name for the test merchant.
    #[serde(rename = "companyName")]
    pub company_name: String,
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,

}

/// Models basic information needed to create a gateway merchant.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AddGatewayMerchantRequest {
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The merchant profile to be boarded.
    #[serde(rename = "profile")]
    pub profile: MerchantProfile,
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,

}

/// Models a request for information about the merchant profile.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantProfileRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The merchant id. Optional for merchant scoped requests.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,

}

/// Models a request related to a platform configuration.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantPlatformRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The platform configuration id.
    #[serde(rename = "platformId")]
    pub platform_id: String,

}

/// Models a request for adding a new user to a merchant account.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InviteMerchantUserRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The merchant id. Optional for merchant scoped requests.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    /// The email address of the user.
    #[serde(rename = "email")]
    pub email: String,
    /// The first name of the new user.
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// The last name of the new user.
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// An optional array of role codes that will be assigned to the user. If omitted defaults to
/// the default merchant role.
    #[serde(rename = "roles")]
    pub roles: Option<Vec<String>>,

}

/// Models a physical address.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Address {
    /// The first line of the street address.
    #[serde(rename = "address1")]
    pub address_1: String,
    /// The second line of the street address.
    #[serde(rename = "address2")]
    pub address_2: String,
    /// The city associated with the street address.
    #[serde(rename = "city")]
    pub city: String,
    /// The state or province associated with the street address.
    #[serde(rename = "stateOrProvince")]
    pub state_or_province: String,
    /// The postal code associated with the street address.
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    /// The ISO country code associated with the street address.
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// The latitude component of the address's GPS coordinates.
    #[serde(rename = "latitude")]
    pub latitude: f64,
    /// The longitude component of the address's GPS coordinates.
    #[serde(rename = "longitude")]
    pub longitude: f64,

}

/// Models a merchant profile.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantProfile {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// That the response came from the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The merchant id.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    /// The primary bank mid.
    #[serde(rename = "bankMid")]
    pub bank_mid: String,
    /// The merchant's company name.
    #[serde(rename = "companyName")]
    pub company_name: String,
    /// The dba name of the merchant.
    #[serde(rename = "dbaName")]
    pub dba_name: String,
    /// The name the merchant prefers on payment link invoices.
    #[serde(rename = "invoiceName")]
    pub invoice_name: String,
    /// The contact name for the merchant.
    #[serde(rename = "contactName")]
    pub contact_name: String,
    /// The contact number for the merchant.
    #[serde(rename = "contactNumber")]
    pub contact_number: String,
    /// The location name.
    #[serde(rename = "locationName")]
    pub location_name: String,
    /// The store number.
    #[serde(rename = "storeNumber")]
    pub store_number: String,
    /// The partner assigne reference for this merchant.
    #[serde(rename = "partnerRef")]
    pub partner_ref: String,
    /// The merchant's local time zone.
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    /// The batch close time in the merchant's time zone.
    #[serde(rename = "batchCloseTime")]
    pub batch_close_time: String,
    /// The terminal firmware update time.
    #[serde(rename = "terminalUpdateTime")]
    pub terminal_update_time: String,
    /// Flag indicating whether or not the batch automatically closes.
    #[serde(rename = "autoBatchClose")]
    pub auto_batch_close: bool,
    /// Flag indicating whether or not batch closure emails should be automatically sent.
    #[serde(rename = "disableBatchEmails")]
    pub disable_batch_emails: bool,
    /// Flag indicating whether or not pin entry is enabled.
    #[serde(rename = "pinEnabled")]
    pub pin_enabled: bool,
    /// Flag indicating whether or not cash back is enabled.
    #[serde(rename = "cashBackEnabled")]
    pub cash_back_enabled: bool,
    /// Flag indicating whether or not store and forward is enabled.
    #[serde(rename = "storeAndForwardEnabled")]
    pub store_and_forward_enabled: bool,
    /// Flag indicating whether or not partial authorizations are supported for this
/// merchant.
    #[serde(rename = "partialAuthEnabled")]
    pub partial_auth_enabled: bool,
    /// Flag indicating whether or not this merchant support split settlement.
    #[serde(rename = "splitBankAccountsEnabled")]
    pub split_bank_accounts_enabled: bool,
    /// Floor limit for store and forward transactions.
    #[serde(rename = "storeAndForwardFloorLimit")]
    pub store_and_forward_floor_limit: String,
    /// The blockchyp public key for this merchant.
    #[serde(rename = "publicKey")]
    pub public_key: String,
    /// The underwriting/processing status for the the merchant.
    #[serde(rename = "status")]
    pub status: String,
    /// Enables cash discount or surcharging.
    #[serde(rename = "cashDiscountEnabled")]
    pub cash_discount_enabled: bool,
    /// The post transaction survey timeout in seconds.
    #[serde(rename = "surveyTimeout")]
    pub survey_timeout: i32,
    /// Time a transaction result is displayed on a terminal before the terminal is
/// automatically cleared in seconds.
    #[serde(rename = "cooldownTimeout")]
    pub cooldown_timeout: i32,
    /// That tips are enabled for a merchant account.
    #[serde(rename = "tipEnabled")]
    pub tip_enabled: bool,
    /// That tips should be automatically prompted for after charge and preauth
/// transactions.
    #[serde(rename = "promptForTip")]
    pub prompt_for_tip: bool,
    /// Three default values for tips. Can be provided as a percentage if a percent sign is
/// provided. Otherwise the values are assumed to be basis points.
    #[serde(rename = "tipDefaults")]
    pub tip_defaults: Option<Vec<String>>,
    /// Four default values for cashback prompts.
    #[serde(rename = "cashbackPresets")]
    pub cashback_presets: Option<Vec<String>>,
    /// That EBT cards are enabled.
    #[serde(rename = "ebtEnabled")]
    pub ebt_enabled: bool,
    /// That refunds without transaction references are permitted.
    #[serde(rename = "freeRangeRefundsEnabled")]
    pub free_range_refunds_enabled: bool,
    /// That pin bypass is enabled.
    #[serde(rename = "pinBypassEnabled")]
    pub pin_bypass_enabled: bool,
    /// That gift cards are disabled.
    #[serde(rename = "giftCardsDisabled")]
    pub gift_cards_disabled: bool,
    /// Disables terms and conditions pages in the merchant UI.
    #[serde(rename = "tcDisabled")]
    pub tc_disabled: bool,
    /// That digital signature capture is enabled.
    #[serde(rename = "digitalSignaturesEnabled")]
    pub digital_signatures_enabled: bool,
    /// That transactions should auto-reverse when signatures are refused.
    #[serde(rename = "digitalSignatureReversal")]
    pub digital_signature_reversal: bool,
    /// The address to be used for billing correspondence.
    #[serde(rename = "billingAddress")]
    pub billing_address: Address,
    /// The address to be used for shipping.
    #[serde(rename = "shippingAddress")]
    pub shipping_address: Address,
    /// That Visa cards are supported.
    #[serde(rename = "visa")]
    pub visa: bool,
    /// That MasterCard is supported.
    #[serde(rename = "masterCard")]
    pub master_card: bool,
    /// That American Express is supported.
    #[serde(rename = "amex")]
    pub amex: bool,
    /// That Discover cards are supported.
    #[serde(rename = "discover")]
    pub discover: bool,
    /// That JCB (Japan Card Bureau) cards are supported.
    #[serde(rename = "jcb")]
    pub jcb: bool,
    /// That China Union Pay cards are supported.
    #[serde(rename = "unionPay")]
    pub union_pay: bool,
    /// That contactless EMV cards are supported.
    #[serde(rename = "contactlessEmv")]
    pub contactless_emv: bool,
    /// That manual card entry is enabled.
    #[serde(rename = "manualEntryEnabled")]
    pub manual_entry_enabled: bool,
    /// Requires a zip code to be entered for manually entered transactions.
    #[serde(rename = "manualEntryPromptZip")]
    pub manual_entry_prompt_zip: bool,
    /// Requires a street number to be entered for manually entered transactions.
    #[serde(rename = "manualEntryPromptStreetNumber")]
    pub manual_entry_prompt_street_number: bool,
    /// That this merchant is boarded on BlockChyp in gateway only mode.
    #[serde(rename = "gatewayOnly")]
    pub gateway_only: bool,
    /// Bank accounts for split bank account merchants.
    #[serde(rename = "bankAccounts")]
    pub bank_accounts: Option<Vec<BankAccount>>,
    /// That a merchant is allowed to send a surcharge amount directly to the gateway.
    #[serde(rename = "passthroughSurchargeEnabled")]
    pub passthrough_surcharge_enabled: bool,
    /// That CVV verification is enabled for manually entered transactions.
    #[serde(rename = "cvvVerificationEnabled")]
    pub cvv_verification_enabled: bool,
    /// That CVV mismatch (N) responses should be declined.
    #[serde(rename = "cvvVerificationNEnabled")]
    pub cvv_verification_nenabled: bool,
    /// That CVV not processed (P) responses should be declined.
    #[serde(rename = "cvvVerificationPEnabled")]
    pub cvv_verification_penabled: bool,
    /// That CVV should be on card but is not indicated (S) responses should be declined.
    #[serde(rename = "cvvVerificationSEnabled")]
    pub cvv_verification_senabled: bool,
    /// That issuer not certified or has not provided encryption key (U) responses should be
/// declined.
    #[serde(rename = "cvvVerificationUEnabled")]
    pub cvv_verification_uenabled: bool,
    /// That the merchant follows the partner's CVV settings.
    #[serde(rename = "followPartnerCvvSettings")]
    pub follow_partner_cvv_settings: bool,
    /// The AVS (Address Verification Service) rule to apply. Allowed values are
/// 'allow_all', 'require_full_match', 'require_zip_match',
/// 'require_address_match'. If avsRule is empty, then merchant follows partner
/// setting.
    #[serde(rename = "avsRule")]
    pub avs_rule: String,
    /// That the merchant follows the partner's AVS settings.
    #[serde(rename = "followPartnerAvsSettings")]
    pub follow_partner_avs_settings: bool,
    /// Flag indicating whether or not account updater is enrolled. Note that only merchant's
/// whose partner is enrolled will be processed by the account updater.
    #[serde(rename = "accountUpdaterEnrolled")]
    pub account_updater_enrolled: bool,
    /// Whether the merchant should bypass an auth with TSYS on Enrollment.
    #[serde(rename = "bypassEnrollAuthEnabled")]
    pub bypass_enroll_auth_enabled: bool,

}

/// Models a response for a single merchant profile.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantProfileResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// That the response came from the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The merchant id.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    /// The primary bank mid.
    #[serde(rename = "bankMid")]
    pub bank_mid: String,
    /// The merchant's company name.
    #[serde(rename = "companyName")]
    pub company_name: String,
    /// The dba name of the merchant.
    #[serde(rename = "dbaName")]
    pub dba_name: String,
    /// The name the merchant prefers on payment link invoices.
    #[serde(rename = "invoiceName")]
    pub invoice_name: String,
    /// The contact name for the merchant.
    #[serde(rename = "contactName")]
    pub contact_name: String,
    /// The contact number for the merchant.
    #[serde(rename = "contactNumber")]
    pub contact_number: String,
    /// The location name.
    #[serde(rename = "locationName")]
    pub location_name: String,
    /// The store number.
    #[serde(rename = "storeNumber")]
    pub store_number: String,
    /// The partner assigne reference for this merchant.
    #[serde(rename = "partnerRef")]
    pub partner_ref: String,
    /// The merchant's local time zone.
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    /// The batch close time in the merchant's time zone.
    #[serde(rename = "batchCloseTime")]
    pub batch_close_time: String,
    /// The terminal firmware update time.
    #[serde(rename = "terminalUpdateTime")]
    pub terminal_update_time: String,
    /// Flag indicating whether or not the batch automatically closes.
    #[serde(rename = "autoBatchClose")]
    pub auto_batch_close: bool,
    /// Flag indicating whether or not batch closure emails should be automatically sent.
    #[serde(rename = "disableBatchEmails")]
    pub disable_batch_emails: bool,
    /// Flag indicating whether or not pin entry is enabled.
    #[serde(rename = "pinEnabled")]
    pub pin_enabled: bool,
    /// Flag indicating whether or not cash back is enabled.
    #[serde(rename = "cashBackEnabled")]
    pub cash_back_enabled: bool,
    /// Flag indicating whether or not store and forward is enabled.
    #[serde(rename = "storeAndForwardEnabled")]
    pub store_and_forward_enabled: bool,
    /// Flag indicating whether or not partial authorizations are supported for this
/// merchant.
    #[serde(rename = "partialAuthEnabled")]
    pub partial_auth_enabled: bool,
    /// Flag indicating whether or not this merchant support split settlement.
    #[serde(rename = "splitBankAccountsEnabled")]
    pub split_bank_accounts_enabled: bool,
    /// Floor limit for store and forward transactions.
    #[serde(rename = "storeAndForwardFloorLimit")]
    pub store_and_forward_floor_limit: String,
    /// The blockchyp public key for this merchant.
    #[serde(rename = "publicKey")]
    pub public_key: String,
    /// The underwriting/processing status for the the merchant.
    #[serde(rename = "status")]
    pub status: String,
    /// Enables cash discount or surcharging.
    #[serde(rename = "cashDiscountEnabled")]
    pub cash_discount_enabled: bool,
    /// The post transaction survey timeout in seconds.
    #[serde(rename = "surveyTimeout")]
    pub survey_timeout: i32,
    /// Time a transaction result is displayed on a terminal before the terminal is
/// automatically cleared in seconds.
    #[serde(rename = "cooldownTimeout")]
    pub cooldown_timeout: i32,
    /// That tips are enabled for a merchant account.
    #[serde(rename = "tipEnabled")]
    pub tip_enabled: bool,
    /// That tips should be automatically prompted for after charge and preauth
/// transactions.
    #[serde(rename = "promptForTip")]
    pub prompt_for_tip: bool,
    /// Three default values for tips. Can be provided as a percentage if a percent sign is
/// provided. Otherwise the values are assumed to be basis points.
    #[serde(rename = "tipDefaults")]
    pub tip_defaults: Option<Vec<String>>,
    /// Four default values for cashback prompts.
    #[serde(rename = "cashbackPresets")]
    pub cashback_presets: Option<Vec<String>>,
    /// That EBT cards are enabled.
    #[serde(rename = "ebtEnabled")]
    pub ebt_enabled: bool,
    /// That refunds without transaction references are permitted.
    #[serde(rename = "freeRangeRefundsEnabled")]
    pub free_range_refunds_enabled: bool,
    /// That pin bypass is enabled.
    #[serde(rename = "pinBypassEnabled")]
    pub pin_bypass_enabled: bool,
    /// That gift cards are disabled.
    #[serde(rename = "giftCardsDisabled")]
    pub gift_cards_disabled: bool,
    /// Disables terms and conditions pages in the merchant UI.
    #[serde(rename = "tcDisabled")]
    pub tc_disabled: bool,
    /// That digital signature capture is enabled.
    #[serde(rename = "digitalSignaturesEnabled")]
    pub digital_signatures_enabled: bool,
    /// That transactions should auto-reverse when signatures are refused.
    #[serde(rename = "digitalSignatureReversal")]
    pub digital_signature_reversal: bool,
    /// The address to be used for billing correspondence.
    #[serde(rename = "billingAddress")]
    pub billing_address: Address,
    /// The address to be used for shipping.
    #[serde(rename = "shippingAddress")]
    pub shipping_address: Address,
    /// That Visa cards are supported.
    #[serde(rename = "visa")]
    pub visa: bool,
    /// That MasterCard is supported.
    #[serde(rename = "masterCard")]
    pub master_card: bool,
    /// That American Express is supported.
    #[serde(rename = "amex")]
    pub amex: bool,
    /// That Discover cards are supported.
    #[serde(rename = "discover")]
    pub discover: bool,
    /// That JCB (Japan Card Bureau) cards are supported.
    #[serde(rename = "jcb")]
    pub jcb: bool,
    /// That China Union Pay cards are supported.
    #[serde(rename = "unionPay")]
    pub union_pay: bool,
    /// That contactless EMV cards are supported.
    #[serde(rename = "contactlessEmv")]
    pub contactless_emv: bool,
    /// That manual card entry is enabled.
    #[serde(rename = "manualEntryEnabled")]
    pub manual_entry_enabled: bool,
    /// Requires a zip code to be entered for manually entered transactions.
    #[serde(rename = "manualEntryPromptZip")]
    pub manual_entry_prompt_zip: bool,
    /// Requires a street number to be entered for manually entered transactions.
    #[serde(rename = "manualEntryPromptStreetNumber")]
    pub manual_entry_prompt_street_number: bool,
    /// That this merchant is boarded on BlockChyp in gateway only mode.
    #[serde(rename = "gatewayOnly")]
    pub gateway_only: bool,
    /// Bank accounts for split bank account merchants.
    #[serde(rename = "bankAccounts")]
    pub bank_accounts: Option<Vec<BankAccount>>,
    /// That a merchant is allowed to send a surcharge amount directly to the gateway.
    #[serde(rename = "passthroughSurchargeEnabled")]
    pub passthrough_surcharge_enabled: bool,
    /// That CVV verification is enabled for manually entered transactions.
    #[serde(rename = "cvvVerificationEnabled")]
    pub cvv_verification_enabled: bool,
    /// That CVV mismatch (N) responses should be declined.
    #[serde(rename = "cvvVerificationNEnabled")]
    pub cvv_verification_nenabled: bool,
    /// That CVV not processed (P) responses should be declined.
    #[serde(rename = "cvvVerificationPEnabled")]
    pub cvv_verification_penabled: bool,
    /// That CVV should be on card but is not indicated (S) responses should be declined.
    #[serde(rename = "cvvVerificationSEnabled")]
    pub cvv_verification_senabled: bool,
    /// That issuer not certified or has not provided encryption key (U) responses should be
/// declined.
    #[serde(rename = "cvvVerificationUEnabled")]
    pub cvv_verification_uenabled: bool,
    /// That the merchant follows the partner's CVV settings.
    #[serde(rename = "followPartnerCvvSettings")]
    pub follow_partner_cvv_settings: bool,
    /// The AVS (Address Verification Service) rule to apply. Allowed values are
/// 'allow_all', 'require_full_match', 'require_zip_match',
/// 'require_address_match'. If avsRule is empty, then merchant follows partner
/// setting.
    #[serde(rename = "avsRule")]
    pub avs_rule: String,
    /// That the merchant follows the partner's AVS settings.
    #[serde(rename = "followPartnerAvsSettings")]
    pub follow_partner_avs_settings: bool,
    /// Flag indicating whether or not account updater is enrolled. Note that only merchant's
/// whose partner is enrolled will be processed by the account updater.
    #[serde(rename = "accountUpdaterEnrolled")]
    pub account_updater_enrolled: bool,
    /// Whether the merchant should bypass an auth with TSYS on Enrollment.
    #[serde(rename = "bypassEnrollAuthEnabled")]
    pub bypass_enroll_auth_enabled: bool,

}

/// Models meta data about a merchant bank account.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    /// The account identifier to be used with authorization requests.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the account.
    #[serde(rename = "name")]
    pub name: String,
    /// The purpose of the account.
    #[serde(rename = "purpose")]
    pub purpose: String,
    /// The masked account number.
    #[serde(rename = "maskedAccountNumber")]
    pub masked_account_number: String,

}

/// Returns a list of queued transactions on a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ListQueuedTransactionsRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,

}

/// A list of queued transactions on a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ListQueuedTransactionsResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// A list of queued transactions on the terminal.
    #[serde(rename = "transactionRefs")]
    pub transaction_refs: Option<Vec<String>>,

}

/// Deletes one or all transactions from a terminal queue.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteQueuedTransactionRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName", default)]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,
    /// A transaction reference string of the transaction to delete. Passing `*` will clear
/// all queued transactions.
    #[serde(rename = "transactionRef")]
    pub transaction_ref: String,

}

/// The response to a delete queued transaction request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteQueuedTransactionResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,

}

/// Deletes a customer record.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteCustomerRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The ID of the customer to delete.
    #[serde(rename = "customerId")]
    pub customer_id: String,

}

/// The response to a delete customer request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteCustomerResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,

}

/// Deletes a payment token.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteTokenRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The token to delete.
    #[serde(rename = "token")]
    pub token: String,

}

/// The response to a delete token request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteTokenResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,

}

/// Links a payment token with a customer record.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LinkTokenRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The token to delete.
    #[serde(rename = "token")]
    pub token: String,
    /// BlockChyp assigned customer id.
    #[serde(rename = "customerId")]
    pub customer_id: String,

}

/// Removes a link between a payment token with a customer record, if one exists.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UnlinkTokenRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef", default)]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard", default)]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef", default)]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount", default)]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase", default)]
    pub test_case: String,
    /// The token to delete.
    #[serde(rename = "token")]
    pub token: String,
    /// BlockChyp assigned customer id.
    #[serde(rename = "customerId")]
    pub customer_id: String,

}

/// Fields for HSA/FSA transactions.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HealthcareMetadata {
    /// A list of healthcare categories in the transaction.
    #[serde(rename = "types")]
    pub types: Option<Vec<HealthcareGroup>>,
    /// That the purchased items were verified against an Inventory Information Approval
/// System (IIAS).
    #[serde(rename = "iiasVerified")]
    pub iias_verified: bool,
    /// That the transaction is exempt from IIAS verification.
    #[serde(rename = "iiasExempt")]
    pub iias_exempt: bool,

}

/// A group of fields for a specific type of healthcare.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HealthcareGroup {
    /// The type of healthcare cost.
    #[serde(rename = "type")]
    pub type_yo: HealthcareType,
    /// The amount of this type.
    #[serde(rename = "amount")]
    pub amount: String,
    /// The provider ID used for Mastercard and Discover IIAS requests.
    #[serde(rename = "providerId")]
    pub provider_id: String,
    /// The service type code used for Mastercard and Discover IIAS requests.
    #[serde(rename = "serviceTypeCode")]
    pub service_type_code: String,
    /// Thr payer ID/carrier ID used for Mastercard and Discover IIAS requests.
    #[serde(rename = "payerOrCarrierId")]
    pub payer_or_carrier_id: String,
    /// The approval or reject reason code used for Mastercard and Discover IIAS requests.
    #[serde(rename = "approvalRejectReasonCode")]
    pub approval_reject_reason_code: String,

}

/// Models a request for merchant information.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetMerchantsRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to return test or live merchants.
    #[serde(rename = "test")]
    pub test: bool,
    /// Max to be returned in a single page. Defaults to the system max of 250.
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    /// Starting index for paged results. Defaults to zero.
    #[serde(rename = "startIndex")]
    pub start_index: i32,

}

/// The results for a merchant list request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetMerchantsResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Whether or not these results are for test or live merchants.
    #[serde(rename = "test")]
    pub test: bool,
    /// Max to be returned in a single page. Defaults to the system max of 250.
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    /// Starting index for paged results. Defaults to zero.
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// Total number of results accessible through paging.
    #[serde(rename = "resultCount")]
    pub result_count: i32,
    /// Merchants in the current page of results.
    #[serde(rename = "merchants")]
    pub merchants: Option<Vec<MerchantProfileResponse>>,

}

/// The results for a merchant users list.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantUsersResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Whether or not these results are for test or live merchants.
    #[serde(rename = "test")]
    pub test: bool,
    /// Users and pending invites associated with the merchant.
    #[serde(rename = "results")]
    pub results: Option<Vec<MerchantUser>>,

}

/// Details about a merchant user.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantUser {
    /// Whether or not these results are for test or live merchants.
    #[serde(rename = "test")]
    pub test: bool,
    /// The user's primary key.
    #[serde(rename = "id")]
    pub id: String,
    /// The user's first name.
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// The user's last name.
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// The user's email address.
    #[serde(rename = "email")]
    pub email: String,
    /// The user account status.
    #[serde(rename = "status")]
    pub status: String,
    /// The type of user account.
    #[serde(rename = "type")]
    pub type_yo: String,
    /// The role codes assigned to this user.
    #[serde(rename = "roles")]
    pub roles: Option<Vec<String>>,
    /// Whether or not this user account is locked.
    #[serde(rename = "locked")]
    pub locked: bool,

}

/// The results for a merchant platforms inquiry.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantPlatformsResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Whether or not these results are for test or live merchants.
    #[serde(rename = "test")]
    pub test: bool,
    /// Enumerates merchant platform settings.
    #[serde(rename = "results")]
    pub results: Option<Vec<MerchantPlatform>>,

}

/// Used to up platform configuration for gateway merchants.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateMerchantPlatformRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The merchant platform configuration.
    #[serde(rename = "platform")]
    pub platform: MerchantPlatform,

}

/// Echoes back the state of the current platform configuration after a change.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateMerchantPlatformResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The current platform configuration.
    #[serde(rename = "platform")]
    pub platform: MerchantPlatform,

}

/// Details about a merchant board platform configuration.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantPlatform {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Primary identifier for a given platform configuration.
    #[serde(rename = "id")]
    pub id: String,
    /// That a platform configuration is disabled.
    #[serde(rename = "disabled")]
    pub disabled: bool,
    /// BlockChyp's code for the boarding platform.
    #[serde(rename = "platformCode")]
    pub platform_code: String,
    /// The platform's priority in a multi platform setup.
    #[serde(rename = "priority")]
    pub priority: i32,
    /// An optional field specifying the merchant's card brand registration record.
    #[serde(rename = "registrationId")]
    pub registration_id: String,
    /// The merchant's primary identifier.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    /// The merchant id assigned by the acquiring bank.
    #[serde(rename = "acquirerMid")]
    pub acquirer_mid: String,
    /// Free form notes description the purpose or intent behind the platform configuration.
    #[serde(rename = "notes")]
    pub notes: String,
    /// The optional entry method code if a platform should only be used for specific entry
/// methods. Leave blank for 'all'.
    #[serde(rename = "entryMethod")]
    pub entry_method: String,
    /// The date the platform configuration was first created.
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// The date the platform configuration was last modified.
    #[serde(rename = "lastChange")]
    pub last_change: String,
    /// A map of configuration values specific to the boarding platform. These are not
/// published. Contact your BlockChyp rep for supported values.
    #[serde(rename = "configMap", default)]
    pub config_map: HashMap<String, String>,

}

/// Models a terminal profile request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalProfileRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,

}

/// Models a terminal profile response.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalProfileResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Enumerates all terminal profiles in the response.
    #[serde(rename = "results")]
    pub results: Option<Vec<TerminalProfile>>,

}

/// Models a terminal deactivation request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalDeactivationRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The terminal name assigned to the terminal.
    #[serde(rename = "terminalName")]
    pub terminal_name: String,
    /// The id assigned by BlockChyp to the terminal.
    #[serde(rename = "terminalId")]
    pub terminal_id: String,

}

/// Models a terminal activation request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalActivationRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The optional merchant id.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    /// The terminal activation code displayed on the terminal.
    #[serde(rename = "activationCode")]
    pub activation_code: String,
    /// The name to be assigned to the terminal. Must be unique for the merchant account.
    #[serde(rename = "terminalName")]
    pub terminal_name: String,
    /// That the terminal should be activated in cloud relay mode.
    #[serde(rename = "cloudRelay")]
    pub cloud_relay: bool,

}

/// Details about a merchant board platform configuration.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalProfile {
    /// Primary identifier for a given terminal.
    #[serde(rename = "id")]
    pub id: String,
    /// The terminal's local IP address.
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    /// The name assigned to the terminal during activation.
    #[serde(rename = "terminalName")]
    pub terminal_name: String,
    /// The terminal type.
    #[serde(rename = "terminalType")]
    pub terminal_type: String,
    /// The terminal type display string.
    #[serde(rename = "terminalTypeDisplayString")]
    pub terminal_type_display_string: String,
    /// The current firmware version deployed on the terminal.
    #[serde(rename = "blockChypFirmwareVersion")]
    pub block_chyp_firmware_version: String,
    /// Whether or not the terminal is configured for cloud relay.
    #[serde(rename = "cloudBased")]
    pub cloud_based: bool,
    /// The terminal's elliptic curve public key.
    #[serde(rename = "publicKey")]
    pub public_key: String,
    /// The manufacturer's serial number.
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    /// Whether or not the terminal is currently online.
    #[serde(rename = "online")]
    pub online: bool,
    /// The date and time the terminal was first brought online.
    #[serde(rename = "since")]
    pub since: String,
    /// The total memory on the terminal.
    #[serde(rename = "totalMemory")]
    pub total_memory: i32,
    /// The storage on the terminal.
    #[serde(rename = "totalStorage")]
    pub total_storage: i32,
    /// The available (unused) memory on the terminal.
    #[serde(rename = "availableMemory")]
    pub available_memory: i32,
    /// The available (unused) storage on the terminal.
    #[serde(rename = "availableStorage")]
    pub available_storage: i32,
    /// The memory currently in use on the terminal.
    #[serde(rename = "usedMemory")]
    pub used_memory: i32,
    /// The storage currently in use on the terminal.
    #[serde(rename = "usedStorage")]
    pub used_storage: i32,
    /// The branding asset currently displayed on the terminal.
    #[serde(rename = "brandingPreview")]
    pub branding_preview: String,
    /// The id of the terminal group to which the terminal belongs, if any.
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// The name of the terminal group to which the terminal belongs, if any.
    #[serde(rename = "groupName")]
    pub group_name: String,

}

/// Models a full terms and conditions template.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TermsAndConditionsTemplate {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Primary identifier for a given template.
    #[serde(rename = "id")]
    pub id: String,
    /// An alias or code used to refer to a template.
    #[serde(rename = "alias")]
    pub alias: String,
    /// The name of the template. Displayed as the agreement title on the terminal.
    #[serde(rename = "name")]
    pub name: String,
    /// The full text of the agreement template.
    #[serde(rename = "content")]
    pub content: String,

}

/// Models a request to retrieve or manipulate terms and conditions data.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TermsAndConditionsTemplateRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Id of a single template.
    #[serde(rename = "templateId")]
    pub template_id: String,

}

/// Models a set of templates responsive to a request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TermsAndConditionsTemplateResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Results responsive to a request.
    #[serde(rename = "results")]
    pub results: Option<Vec<TermsAndConditionsTemplate>>,
    /// An optional timeout override.
    #[serde(rename = "timeout")]
    pub timeout: i32,

}

/// Models a Terms and Conditions history request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TermsAndConditionsLogRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The identifier of the log entry to be returned for single result requests.
    #[serde(rename = "logEntryId")]
    pub log_entry_id: String,
    /// Optional transaction id if only log entries related to a transaction should be
/// returned.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// Max to be returned in a single page. Defaults to the system max of 250.
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    /// Starting index for paged results. Defaults to zero.
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// An optional start date for filtering response data.
    #[serde(rename = "startDate")]
    pub start_date: String,
    /// An optional end date for filtering response data.
    #[serde(rename = "endDate")]
    pub end_date: String,

}

/// Models a Terms and Conditions history request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TermsAndConditionsLogResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Optional transaction id if only log entries related to a transaction should be
/// returned.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// Max to be returned in a single page. Defaults to the system max of 250.
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    /// Starting index for paged results. Defaults to zero.
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// Total number of results accessible through paging.
    #[serde(rename = "resultCount")]
    pub result_count: i32,
    /// The full result set responsive to the original request, subject to pagination limits.
    #[serde(rename = "results")]
    pub results: Option<Vec<TermsAndConditionsLogEntry>>,

}

/// Models a Terms and Conditions log entry.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TermsAndConditionsLogEntry {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Internal id for a Terms and Conditions entry.
    #[serde(rename = "id")]
    pub id: String,
    /// Id of the terminal that captured this terms and conditions entry.
    #[serde(rename = "terminalId")]
    pub terminal_id: String,
    /// Name of the terminal that captured this terms and conditions entry.
    #[serde(rename = "terminalName")]
    pub terminal_name: String,
    /// A flag indicating whether or not the terminal was a test terminal.
    #[serde(rename = "test")]
    pub test: bool,
    /// Date and time the terms and conditions acceptance occurred.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// Optional transaction ref if the terms and conditions was associated with a
/// transaction.
    #[serde(rename = "transactionRef")]
    pub transaction_ref: String,
    /// Optional transaction id if only log entries related to a transaction should be
/// returned.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// Alias of the terms and conditions template used for this entry, if any.
    #[serde(rename = "alias")]
    pub alias: String,
    /// Title of the document displayed on the terminal at the time of capture.
    #[serde(rename = "name")]
    pub name: String,
    /// Full text of the document agreed to at the time of signature capture.
    #[serde(rename = "content")]
    pub content: String,
    /// First 32 characters of the full text. Used to support user interfaces that show
/// summaries.
    #[serde(rename = "contentLeader")]
    pub content_leader: String,
    /// A flag that indicates whether or not a signature has been captured.
    #[serde(rename = "hasSignature")]
    pub has_signature: bool,
    /// The image format to be used for returning signatures.
    #[serde(rename = "sigFormat", default)]
    pub sig_format: SignatureFormat,
    /// The base 64 encoded signature image if the format requested.
    #[serde(rename = "signature")]
    pub signature: String,

}

/// Models a survey question.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SurveyQuestion {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Internal id for a survey question.
    #[serde(rename = "id")]
    pub id: String,
    /// Ordinal number indicating the position of the survey question in the post transaction
/// sequence.
    #[serde(rename = "ordinal")]
    pub ordinal: i32,
    /// Determines whether or not the question will be presented post transaction.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The full text of the transaction.
    #[serde(rename = "questionText")]
    pub question_text: String,
    /// The type of question. Valid values are 'yes_no' and 'scaled'.
    #[serde(rename = "questionType")]
    pub question_type: String,
    /// The total number of transactions processed during the query period if results are
/// requested.
    #[serde(rename = "transactionCount", default)]
    pub transaction_count: i32,
    /// The total number of responses during the query period if results are requested.
    #[serde(rename = "responseCount", default)]
    pub response_count: i32,
    /// The response rate, expressed as a ratio, if results are requested.
    #[serde(rename = "responseRate", default)]
    pub response_rate: f64,
    /// The set of response data points.
    #[serde(rename = "responses")]
    pub responses: Option<Vec<SurveyDataPoint>>,

}

/// Models a request to retrieve or manipulate survey questions.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SurveyQuestionRequest {
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Id of a single question.
    #[serde(rename = "questionId")]
    pub question_id: String,
    /// An optional timeout override.
    #[serde(rename = "timeout")]
    pub timeout: i32,

}

/// Models a survey question response.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SurveyQuestionResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The full result set responsive to the original request.
    #[serde(rename = "results")]
    pub results: Option<Vec<SurveyQuestion>>,

}

/// Models a request to retrieve or manipulate survey questions.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SurveyDataPoint {
    /// A unique identifier for a specific answer type.
    #[serde(rename = "answerKey")]
    pub answer_key: String,
    /// A narrative description of the answer.
    #[serde(rename = "answerDescription")]
    pub answer_description: String,
    /// The number of responses.
    #[serde(rename = "responseCount")]
    pub response_count: i32,
    /// Response rate as a percentage of total transactions.
    #[serde(rename = "responsePercentage")]
    pub response_percentage: f64,
    /// The average transaction amount for a given answer.
    #[serde(rename = "averageTransaction")]
    pub average_transaction: f64,

}

/// Models a request to retrieve survey results.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SurveyResultsRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Id of a single question.
    #[serde(rename = "questionId")]
    pub question_id: String,
    /// An optional start date for filtering response data.
    #[serde(rename = "startDate")]
    pub start_date: String,
    /// An optional end date for filtering response data.
    #[serde(rename = "endDate")]
    pub end_date: String,

}

/// Models a request to retrieve survey results.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Id used to identify the media asset.
    #[serde(rename = "id")]
    pub id: String,
    /// The original filename assigned to the media asset.
    #[serde(rename = "originalFile")]
    pub original_file: String,
    /// The descriptive name of the media asset.
    #[serde(rename = "name")]
    pub name: String,
    /// A description of the media asset and its purpose.
    #[serde(rename = "description")]
    pub description: String,
    /// An array of tags associated with a media asset.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The url for the full resolution versio of the media file.
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    /// The url for to the thumbnail of an image.
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
    /// An identifier used to flag video files.
    #[serde(rename = "video")]
    pub video: bool,

}

/// Models information needed to process a file upload.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UploadMetadata {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Optional id used to track status and progress of an upload while in progress.
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    /// The size of the file to be uploaded in bytes.
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    /// The name of file to be uploaded.
    #[serde(rename = "fileName")]
    pub file_name: String,

}

/// Models the current status of a file upload.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UploadStatus {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Id used to track status and progress of an upload while in progress.
    #[serde(rename = "id")]
    pub id: String,
    /// The media id assigned to the result.
    #[serde(rename = "mediaId")]
    pub media_id: String,
    /// The size of the file to be uploaded in bytes.
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    /// The amount of the file already uploaded.
    #[serde(rename = "uploadedAmount")]
    pub uploaded_amount: i64,
    /// The current status of a file upload.
    #[serde(rename = "status")]
    pub status: String,
    /// Whether or not the upload and associated file processing is complete.
    #[serde(rename = "complete")]
    pub complete: bool,
    /// Whether or not the file is processing. This normally applied to video files undergoing
/// format transcoding.
    #[serde(rename = "processing")]
    pub processing: bool,
    /// Current upload progress rounded to the nearest integer.
    #[serde(rename = "percentage")]
    pub percentage: i32,
    /// The url of a thumbnail for the file, if available.
    #[serde(rename = "thumbnailLocation")]
    pub thumbnail_location: String,

}

/// Used to request the status of a file upload.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UploadStatusRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Id used to track status and progress of an upload while in progress.
    #[serde(rename = "uploadId")]
    pub upload_id: String,

}

/// Models a request to retrieve or manipulate media assets.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MediaRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Id used to track a media asset.
    #[serde(rename = "mediaId")]
    pub media_id: String,

}

/// Models a media library response.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MediaLibraryResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Max to be returned in a single page. Defaults to the system max of 250.
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    /// Starting index for paged results. Defaults to zero.
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// Total number of results accessible through paging.
    #[serde(rename = "resultCount")]
    pub result_count: i32,
    /// Total number of pages.
    #[serde(rename = "pages")]
    pub pages: i32,
    /// Page currently selected through paging.
    #[serde(rename = "currentPage")]
    pub current_page: i32,
    /// Enumerates all media assets available in the context.
    #[serde(rename = "results")]
    pub results: Option<Vec<MediaMetadata>>,

}

/// Models a slide within a slide show.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Slide {
    /// The id for the media asset to be used for this slide. Must be an image.
    #[serde(rename = "mediaId")]
    pub media_id: String,
    /// Position of the slide within the slide show.
    #[serde(rename = "ordinal")]
    pub ordinal: i32,
    /// The fully qualified thumbnail url for the slide.
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,

}

/// Models a media library response.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SlideShow {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The primary id for the slide show.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the slide show.
    #[serde(rename = "name")]
    pub name: String,
    /// Time between slides in seconds.
    #[serde(rename = "delay")]
    pub delay: i32,
    /// Enumerates all slides in the display sequence.
    #[serde(rename = "slides")]
    pub slides: Option<Vec<Slide>>,

}

/// Models a slide show response.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SlideShowResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Max to be returned in a single page. Defaults to the system max of 250.
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    /// Starting index for paged results. Defaults to zero.
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// Total number of results accessible through paging.
    #[serde(rename = "resultCount")]
    pub result_count: i32,
    /// Enumerates all slide shows responsive to the original query.
    #[serde(rename = "results")]
    pub results: Option<Vec<SlideShow>>,

}

/// Models a request to retrieve or manipulate terminal slide shows.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SlideShowRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Id used to track a slide show.
    #[serde(rename = "slideShowId")]
    pub slide_show_id: String,

}

/// Models a request to retrieve or manipulate terminal slide shows.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BrandingAssetRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Id used to track a branding asset.
    #[serde(rename = "assetId")]
    pub asset_id: String,

}

/// Models the priority and display settings for terminal media.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BrandingAsset {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Id used to track a branding asset.
    #[serde(rename = "id")]
    pub id: String,
    /// The id owner of the tenant who owns the branding asset.
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    /// The terminal id if this branding asset is specific to a single terminal.
    #[serde(rename = "terminalId")]
    pub terminal_id: String,
    /// The terminal group id if this branding asset is specific to a terminal group.
    #[serde(rename = "terminalGroupId")]
    pub terminal_group_id: String,
    /// The merchant id associated with this branding asset.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    /// The organization id associated with this branding asset.
    #[serde(rename = "organizationId")]
    pub organization_id: String,
    /// The partner id associated with this branding asset.
    #[serde(rename = "partnerId")]
    pub partner_id: String,
    /// The slide show associated with this branding asset, if any. A branding asset can
/// reference a slide show or media asset, but not both.
    #[serde(rename = "slideShowId")]
    pub slide_show_id: String,
    /// The media id associated with this branding asset, if any. A branding asset can
/// reference a slide show or media asset, but not both.
    #[serde(rename = "mediaId")]
    pub media_id: String,
    /// Applies standard margins to images displayed on terminals. Usually the best option
/// for logos.
    #[serde(rename = "padded")]
    pub padded: bool,
    /// The start date if this asset should be displayed based on a schedule. Format:
/// MM/DD/YYYY.
    #[serde(rename = "startDate")]
    pub start_date: String,
    /// The end date if this asset should be displayed based on a schedule. Format: MM/DD/YYYY.
    #[serde(rename = "endDate")]
    pub end_date: String,
    /// An array of days of the week during which a branding asset should be enabled. Days of the
/// week are coded as integers starting with Sunday (0) and ending with Saturday (6).
    #[serde(rename = "daysOfWeek")]
    pub days_of_week: Option<Vec<i8>>,
    /// The start date if this asset should be displayed based on a schedule. Format:
/// MM/DD/YYYY.
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// The end date if this asset should be displayed based on a schedule. Format: MM/DD/YYYY.
    #[serde(rename = "endTime")]
    pub end_time: String,
    /// The ordinal number marking the position of this asset within the branding stack.
    #[serde(rename = "ordinal")]
    pub ordinal: i32,
    /// Enables the asset for display.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// If true, the asset will be displayed in the merchant portal, but not on merchant
/// terminal hardware. Developers will usually want this to always be false.
    #[serde(rename = "preview")]
    pub preview: bool,
    /// Id of the user who created this branding asset, if applicable.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Name of the user who created this branding asset, if applicable.
    #[serde(rename = "userName")]
    pub user_name: String,
    /// The fully qualified URL of the thumbnail image for this branding asset.
    #[serde(rename = "thumbnail")]
    pub thumbnail: String,
    /// The time and date this asset was last modified.
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    /// A field for notes related to a branding asset.
    #[serde(rename = "notes")]
    pub notes: String,
    /// If true, the API credentials used to retrieve the branding asset record can be used to
/// update it.
    #[serde(rename = "editable")]
    pub editable: bool,
    /// The type of branding asset.
    #[serde(rename = "assetType")]
    pub asset_type: String,
    /// The type of user or tenant that owns this asset.
    #[serde(rename = "ownerType")]
    pub owner_type: String,
    /// A recommended caption for displaying the owner. Takes into account multiple
/// organization types.
    #[serde(rename = "ownerTypeCaption")]
    pub owner_type_caption: String,
    /// The name of the tenant or entity that owns the branding asset.
    #[serde(rename = "ownerName")]
    pub owner_name: String,
    /// The recommended image to be displayed when rendering a preview of this branding asset.
    #[serde(rename = "previewImage")]
    pub preview_image: String,
    /// A compact narrative string explaining the effective date and time rules for a branding
/// asset.
    #[serde(rename = "narrativeEffectiveDates")]
    pub narrative_effective_dates: String,
    /// A compact narrative string explaining the display period for a branding asset.
    #[serde(rename = "narrativeDisplayPeriod")]
    pub narrative_display_period: String,

}

/// Models a branding asset response.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BrandingAssetResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The id owner of this branding stack.
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    /// The type of user or tenant that owns this branding stack.
    #[serde(rename = "ownerType")]
    pub owner_type: String,
    /// The name of the entity or tenant that owns this branding stack.
    #[serde(rename = "ownerName")]
    pub owner_name: String,
    /// The owner level currently being displayed.
    #[serde(rename = "levelName")]
    pub level_name: String,
    /// A narrative description of the current simulate time.
    #[serde(rename = "narrativeTime")]
    pub narrative_time: String,
    /// The asset currently displayed on the terminal.
    #[serde(rename = "activeAsset")]
    pub active_asset: BrandingAsset,
    /// Enumerates all branding assets in a given credential scope.
    #[serde(rename = "results")]
    pub results: Option<Vec<BrandingAsset>>,

}

/// Models a request to retrieve pricing policy information. The default policy for the
/// merchant is returned if no idea is provided.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PricingPolicyRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// An optional id used to retrieve a specific pricing policy.
    #[serde(rename = "id")]
    pub id: String,
    /// An optional merchant id if this request is submitted via partner credentials.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,

}

/// Models a single set of pricing values for a pricing policy.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    /// The string representation of a per transaction minimum.
    #[serde(rename = "buyRate")]
    pub buy_rate: String,
    /// The string representation of the current fee per transaction.
    #[serde(rename = "current")]
    pub current: String,
    /// The string representation of a per transaction gouge limit.
    #[serde(rename = "limit")]
    pub limit: String,

}

/// Models a the response to a pricing policy request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PricingPolicyResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The id owner of the pricing policy.
    #[serde(rename = "id")]
    pub id: String,
    /// The id of the partner associated with this pricing policy.
    #[serde(rename = "partnerId")]
    pub partner_id: String,
    /// The id of the merchant associated with this pricing policy.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    /// Whether or not a pricing policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The date and time when the pricing policy was created.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The description of the pricing policy.
    #[serde(rename = "description")]
    pub description: String,
    /// Type of pricing policy (flat vs interchange).
    #[serde(rename = "policyType")]
    pub policy_type: String,
    /// The percentage split of the of buy rate markup with BlockChyp.
    #[serde(rename = "partnerMarkupSplit")]
    pub partner_markup_split: String,
    /// The flat rate percentage for standard card present transactions.
    #[serde(rename = "standardFlatRate")]
    pub standard_flat_rate: PricePoint,
    /// The flat rate percentage for debit card transactions.
    #[serde(rename = "debitFlatRate")]
    pub debit_flat_rate: PricePoint,
    /// The flat rate percentage for ecommerce transactions.
    #[serde(rename = "ecommerceFlatRate")]
    pub ecommerce_flat_rate: PricePoint,
    /// The flat rate percentage for keyed/manual transactions.
    #[serde(rename = "keyedFlatRate")]
    pub keyed_flat_rate: PricePoint,
    /// The flat rate percentage for premium (high rewards) card transactions.
    #[serde(rename = "premiumFlatRate")]
    pub premium_flat_rate: PricePoint,
    /// The interchange markup percentage for standard card present transactions.
    #[serde(rename = "standardInterchangeMarkup")]
    pub standard_interchange_markup: PricePoint,
    /// The interchange markup percentage for debit card transactions.
    #[serde(rename = "debitInterchangeMarkup")]
    pub debit_interchange_markup: PricePoint,
    /// The interchange markup percentage for ecommerce transactions.
    #[serde(rename = "ecommerceInterchangeMarkup")]
    pub ecommerce_interchange_markup: PricePoint,
    /// The interchange markup percentage for keyed/manual transactions.
    #[serde(rename = "keyedInterchangeMarkup")]
    pub keyed_interchange_markup: PricePoint,
    /// The interchange markup percentage for premium (high rewards) card transactions.
    #[serde(rename = "premiumInterchangeMarkup")]
    pub premium_interchange_markup: PricePoint,
    /// The transaction fee for standard card present transactions.
    #[serde(rename = "standardTransactionFee")]
    pub standard_transaction_fee: PricePoint,
    /// The transaction fee for debit card transactions.
    #[serde(rename = "debitTransactionFee")]
    pub debit_transaction_fee: PricePoint,
    /// The transaction fee for ecommerce transactions.
    #[serde(rename = "ecommerceTransactionFee")]
    pub ecommerce_transaction_fee: PricePoint,
    /// The transaction fee for keyed/manual transactions.
    #[serde(rename = "keyedTransactionFee")]
    pub keyed_transaction_fee: PricePoint,
    /// The transaction fee for premium (high rewards) card transactions.
    #[serde(rename = "premiumTransactionFee")]
    pub premium_transaction_fee: PricePoint,
    /// The transaction fee for EBT card transactions.
    #[serde(rename = "ebtTransactionFee")]
    pub ebt_transaction_fee: PricePoint,
    /// A flat fee charged per month.
    #[serde(rename = "monthlyFee")]
    pub monthly_fee: PricePoint,
    /// A flat fee charged per year.
    #[serde(rename = "annualFee")]
    pub annual_fee: PricePoint,
    /// The fee per dispute or chargeback.
    #[serde(rename = "chargebackFee")]
    pub chargeback_fee: PricePoint,
    /// The fee per address verification operation.
    #[serde(rename = "avsFee")]
    pub avs_fee: PricePoint,
    /// The fee per batch.
    #[serde(rename = "batchFee")]
    pub batch_fee: PricePoint,
    /// The voice authorization fee.
    #[serde(rename = "voiceAuthFee")]
    pub voice_auth_fee: PricePoint,
    /// The one time account setup fee.
    #[serde(rename = "accountSetupFee")]
    pub account_setup_fee: PricePoint,

}

/// Models a request to retrieve a list of partner statements.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PartnerStatementListRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Optional start date filter for batch history.
    #[serde(rename = "startDate")]
    pub start_date: Option<DateTime<Utc>>,
    /// Optional end date filter for batch history.
    #[serde(rename = "endDate")]
    pub end_date: Option<DateTime<Utc>>,

}

/// Models a basic information about partner statements for use in list or search results.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PartnerStatementSummary {
    /// The id owner of the pricing policy.
    #[serde(rename = "id")]
    pub id: String,
    /// The date the statement was generated.
    #[serde(rename = "statementDate")]
    pub statement_date: DateTime<Utc>,
    /// Total volume in numeric format.
    #[serde(rename = "totalVolume")]
    pub total_volume: f64,
    /// The string formatted total volume on the statement.
    #[serde(rename = "totalVolumeFormatted")]
    pub total_volume_formatted: String,
    /// The total volume on the statement.
    #[serde(rename = "transactionCount")]
    pub transaction_count: i64,
    /// The commission earned on the portfolio during the statement period.
    #[serde(rename = "partnerCommission")]
    pub partner_commission: f64,
    /// The string formatted total volume on the statement.
    #[serde(rename = "partnerCommissionFormatted")]
    pub partner_commission_formatted: String,
    /// The status of the statement.
    #[serde(rename = "status")]
    pub status: String,

}

/// Models results to a partner statement list inquiry.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PartnerStatementListResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The list of statements summaries.
    #[serde(rename = "statements")]
    pub statements: Option<Vec<PartnerStatementSummary>>,

}

/// Models a request to retrieve detailed partner statement information.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PartnerStatementDetailRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Optional start date filter for batch history.
    #[serde(rename = "id")]
    pub id: String,

}

/// Models a response to retrieve detailed partner statement information.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PartnerStatementDetailResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Optional start date filter for batch history.
    #[serde(rename = "id")]
    pub id: String,
    /// The id of the partner associated with the statement.
    #[serde(rename = "partnerId")]
    pub partner_id: String,
    /// The name of the partner associated with the statement.
    #[serde(rename = "partnerName")]
    pub partner_name: String,
    /// The date the statement was generated.
    #[serde(rename = "statementDate")]
    pub statement_date: DateTime<Utc>,
    /// Total volume in numeric format.
    #[serde(rename = "totalVolume")]
    pub total_volume: f64,
    /// The string formatted total volume on the statement.
    #[serde(rename = "totalVolumeFormatted")]
    pub total_volume_formatted: String,
    /// The total volume on the statement.
    #[serde(rename = "transactionCount")]
    pub transaction_count: i64,
    /// The commission earned on the portfolio during the statement period.
    #[serde(rename = "partnerCommission")]
    pub partner_commission: f64,
    /// The string formatted partner commission on the statement.
    #[serde(rename = "partnerCommissionFormatted")]
    pub partner_commission_formatted: String,
    /// The partner commission earned on the portfolio during the statement period as a ratio
/// against volume.
    #[serde(rename = "partnerCommissionsOnVolume")]
    pub partner_commissions_on_volume: f64,
    /// The string formatted version of partner commissions as a percentage of volume.
    #[serde(rename = "partnerCommissionsOnVolumeFormatted")]
    pub partner_commissions_on_volume_formatted: String,
    /// The status of the statement.
    #[serde(rename = "status")]
    pub status: String,
    /// The line item detail associated with the statement.
    #[serde(rename = "lineItems")]
    pub line_items: Option<Vec<PartnerStatementLineItem>>,
    /// The list of adjustments made against the statement, if any.
    #[serde(rename = "adjustments")]
    pub adjustments: Option<Vec<PartnerStatementAdjustment>>,
    /// The list of partner disbursements made against the partner statement.
    #[serde(rename = "disbursements")]
    pub disbursements: Option<Vec<PartnerStatementDisbursement>>,

}

/// Models line item level data for a partner statement.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PartnerStatementLineItem {
    /// The line item id.
    #[serde(rename = "id")]
    pub id: String,
    /// The invoice id for the underlying merchant statement.
    #[serde(rename = "invoiceId")]
    pub invoice_id: String,
    /// The total fees charged to the merchant.
    #[serde(rename = "totalFees")]
    pub total_fees: f64,
    /// The total fees charge formatted as a currency string.
    #[serde(rename = "totalFeesFormatted")]
    pub total_fees_formatted: String,
    /// The total fees charged to the merchant as ratio of volume.
    #[serde(rename = "totalFeesOnVolume")]
    pub total_fees_on_volume: f64,
    /// The total fees charged to the merchant as percentage of volume.
    #[serde(rename = "totalFeesOnVolumeFormatted")]
    pub total_fees_on_volume_formatted: String,
    /// The id of the merchant.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    /// The corporate name of the merchant.
    #[serde(rename = "merchantName")]
    pub merchant_name: String,
    /// The dba name of the merchant.
    #[serde(rename = "dbaName")]
    pub dba_name: String,
    /// The date the statement was generated.
    #[serde(rename = "statementDate")]
    pub statement_date: DateTime<Utc>,
    /// Volume in numeric format.
    #[serde(rename = "volume")]
    pub volume: f64,
    /// The string formatted total volume on the statement.
    #[serde(rename = "volumeFormatted")]
    pub volume_formatted: String,
    /// The total volume on the statement.
    #[serde(rename = "transactionCount")]
    pub transaction_count: i64,
    /// The total interchange fees incurred this period.
    #[serde(rename = "interchange")]
    pub interchange: f64,
    /// The currency formatted form of interchange.
    #[serde(rename = "interchangeFormatted")]
    pub interchange_formatted: String,
    /// The total interchange as a ratio on volume incurred this period.
    #[serde(rename = "interchangeOnVolume")]
    pub interchange_on_volume: f64,
    /// The total interchange as a percentage of volume.
    #[serde(rename = "interchangeOnVolumeFormatted")]
    pub interchange_on_volume_formatted: String,
    /// The total card brand assessments fees incurred this period.
    #[serde(rename = "assessments")]
    pub assessments: f64,
    /// The currency formatted form of card brand assessments.
    #[serde(rename = "assessmentsFormatted")]
    pub assessments_formatted: String,
    /// The total card brand assessments as a ratio on volume incurred this period.
    #[serde(rename = "assessmentsOnVolume")]
    pub assessments_on_volume: f64,
    /// The total card brand assessments as a percentage of volume.
    #[serde(rename = "assessmentsOnVolumeFormatted")]
    pub assessments_on_volume_formatted: String,
    /// The commission earned on the portfolio during the statement period.
    #[serde(rename = "partnerCommission")]
    pub partner_commission: f64,
    /// The string formatted total volume on the statement.
    #[serde(rename = "partnerCommissionFormatted")]
    pub partner_commission_formatted: String,
    /// The total fees charge to the partner due to buy rates.
    #[serde(rename = "buyRate")]
    pub buy_rate: f64,
    /// The currency formatted form of partner buy rate.
    #[serde(rename = "buyRateFormatted")]
    pub buy_rate_formatted: String,
    /// Refers to card brand fees shared between BlockChyp and the partner.
    #[serde(rename = "hardCosts")]
    pub hard_costs: f64,
    /// The currency formatted form of hard costs.
    #[serde(rename = "hardCostsFormatted")]
    pub hard_costs_formatted: String,

}

/// Models details about disbursements related to partner statements.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PartnerStatementDisbursement {
    /// The disbursement id.
    #[serde(rename = "id")]
    pub id: String,
    /// Date and time the disbursement was processed.
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    /// The type of disbursement transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The payment method used to fund the disbursement.
    #[serde(rename = "paymentType")]
    pub payment_type: String,
    /// The masked account number into which funds were deposited.
    #[serde(rename = "maskedPan")]
    pub masked_pan: String,
    /// That payment is still pending.
    #[serde(rename = "pending")]
    pub pending: bool,
    /// That payment is approved.
    #[serde(rename = "approved")]
    pub approved: bool,
    /// A response description from the disbursement payment platform, in any.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The amount disbursed in floating point format.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The currency formatted form of amount.
    #[serde(rename = "amountFormatted")]
    pub amount_formatted: String,

}

/// Models partner statement adjustments.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PartnerStatementAdjustment {
    /// The adjustment id.
    #[serde(rename = "id")]
    pub id: String,
    /// The date and time the disbursement was posted to the account.
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    /// A description of the adjustment.
    #[serde(rename = "description")]
    pub description: String,
    /// The amount in floating point format.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The currency formatted form of amount.
    #[serde(rename = "amountFormatted")]
    pub amount_formatted: String,

}

/// Models a request to retrieve a list of partner statements.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantInvoiceListRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// Optional merchant id for partner scoped requests.
    #[serde(rename = "merchantId")]
    pub merchant_id: Option<String>,
    /// Optional type to filter normal invoices vs statements.
    #[serde(rename = "invoiceType")]
    pub invoice_type: Option<String>,
    /// Optional start date filter for batch history.
    #[serde(rename = "startDate")]
    pub start_date: Option<DateTime<Utc>>,
    /// Optional end date filter for batch history.
    #[serde(rename = "endDate")]
    pub end_date: Option<DateTime<Utc>>,

}

/// Models a response to an invoice list request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantInvoiceListResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The list of invoices returned by the request.
    #[serde(rename = "invoices")]
    pub invoices: Option<Vec<MerchantInvoiceSummary>>,

}

/// Models basic information about a merchant invoice for use in list or search results.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantInvoiceSummary {
    /// The id owner of the invoice.
    #[serde(rename = "id")]
    pub id: String,
    /// The date the statement was generated.
    #[serde(rename = "dateCreated")]
    pub date_created: DateTime<Utc>,
    /// The grand total.
    #[serde(rename = "grandTotal")]
    pub grand_total: f64,
    /// The string formatted grand total.
    #[serde(rename = "grandTotalFormatted")]
    pub grand_total_formatted: String,
    /// The status of the statement.
    #[serde(rename = "status")]
    pub status: String,
    /// Identifies the invoice type.
    #[serde(rename = "invoiceType")]
    pub invoice_type: String,
    /// Whether or not the invoice had been paid.
    #[serde(rename = "paid")]
    pub paid: bool,

}

/// Models a request to retrieve detailed merchant invoice information.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantInvoiceDetailRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The invoice id.
    #[serde(rename = "id")]
    pub id: String,

}

/// Models detailed merchant invoice or statement information.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantInvoiceDetailResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// Optional start date filter for batch history.
    #[serde(rename = "id")]
    pub id: String,
    /// The id of the merchant associated with the statement.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    /// The corporate name of the merchant associated with the statement.
    #[serde(rename = "corporateName")]
    pub corporate_name: String,
    /// The dba name of the merchant associated with the statement.
    #[serde(rename = "dbaName")]
    pub dba_name: String,
    /// The date the statement was generated.
    #[serde(rename = "dateCreated")]
    pub date_created: DateTime<Utc>,
    /// The current status of the invoice.
    #[serde(rename = "status")]
    pub status: String,
    /// The type of invoice (statement or invoice).
    #[serde(rename = "invoiceType")]
    pub invoice_type: String,
    /// The type of pricing used for the invoice (typically flat rate or or interchange plus).
    #[serde(rename = "pricingType")]
    pub pricing_type: String,
    /// Whether or not the invoice has been paid.
    #[serde(rename = "paid")]
    pub paid: bool,
    /// The grand total.
    #[serde(rename = "grandTotal")]
    pub grand_total: f64,
    /// The string formatted grand total.
    #[serde(rename = "grandTotalFormatted")]
    pub grand_total_formatted: String,
    /// The subtotal before shipping and tax.
    #[serde(rename = "subtotal")]
    pub subtotal: f64,
    /// The string formatted subtotal before shipping and tax.
    #[serde(rename = "subotalFormatted")]
    pub subotal_formatted: String,
    /// The total sales tax.
    #[serde(rename = "taxTotal")]
    pub tax_total: f64,
    /// The string formatted total sales tax.
    #[serde(rename = "taxTotalFormatted")]
    pub tax_total_formatted: String,
    /// The total cost of shipping.
    #[serde(rename = "shippingCost")]
    pub shipping_cost: f64,
    /// The string formatted total cost of shipping.
    #[serde(rename = "shippingCostFormatted")]
    pub shipping_cost_formatted: String,
    /// The total unpaid balance on the invoice.
    #[serde(rename = "balanceDue")]
    pub balance_due: f64,
    /// The string formatted unpaid balance on the invoice.
    #[serde(rename = "balanceDueFormatted")]
    pub balance_due_formatted: String,
    /// The shipping or physical address associated with the invoice.
    #[serde(rename = "shippingAddress")]
    pub shipping_address: Option<Address>,
    /// The billing or mailing address associated with the invoice.
    #[serde(rename = "billingAddress")]
    pub billing_address: Option<Address>,
    /// The list of line item details associated with the invoice.
    #[serde(rename = "lineItems")]
    pub line_items: Option<Vec<InvoiceLineItem>>,
    /// The list of payments collected against the invoice.
    #[serde(rename = "payments")]
    pub payments: Option<Vec<InvoicePayment>>,
    /// The list of merchant settlements disbursed during the statement period.
    #[serde(rename = "deposits")]
    pub deposits: Option<Vec<StatementDeposit>>,

}

/// Models a single invoice or merchant statement line item.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceLineItem {
    /// The line item id.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of line item.
    #[serde(rename = "lineType")]
    pub line_type: String,
    /// The product id for standard invoices.
    #[serde(rename = "productId")]
    pub product_id: String,
    /// The quantity associated with the line item.
    #[serde(rename = "quantity")]
    pub quantity: i64,
    /// The description associated with the line item.
    #[serde(rename = "description")]
    pub description: String,
    /// An alternate explanation.
    #[serde(rename = "explanation")]
    pub explanation: String,
    /// The transaction count associated with any transaction based fees.
    #[serde(rename = "transactionCount")]
    pub transaction_count: i64,
    /// The transaction volume associated with any fees.
    #[serde(rename = "volume")]
    pub volume: f64,
    /// The string formatted volume.
    #[serde(rename = "volumeFormatted")]
    pub volume_formatted: String,
    /// The per transaction fee.
    #[serde(rename = "perTransactionFee")]
    pub per_transaction_fee: f64,
    /// The string formatted per transaction fee.
    #[serde(rename = "perTransactionFeeFormatted")]
    pub per_transaction_fee_formatted: String,
    /// The percentage (as floating point ratio) fee assessed on volume.
    #[serde(rename = "transactionPercentage")]
    pub transaction_percentage: f64,
    /// The string formatted transaction fee percentage.
    #[serde(rename = "transactionPercentageFormatted")]
    pub transaction_percentage_formatted: String,
    /// The quantity price associated.
    #[serde(rename = "price")]
    pub price: f64,
    /// The string formatted price associated with a conventional line item.
    #[serde(rename = "priceFormatted")]
    pub price_formatted: String,
    /// The extended price .
    #[serde(rename = "priceExtended")]
    pub price_extended: f64,
    /// The string formatted extended price.
    #[serde(rename = "priceExtendedFormatted")]
    pub price_extended_formatted: String,
    /// The list of nested line items, if any.
    #[serde(rename = "lineItems")]
    pub line_items: Option<Vec<InvoiceLineItem>>,

}

/// Models information about payments against an invoice.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicePayment {
    /// The line item id.
    #[serde(rename = "id")]
    pub id: String,
    /// Timestamp the payment was authorized.
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    /// The type of disbursement transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The payment method used to fund the disbursement.
    #[serde(rename = "paymentType")]
    pub payment_type: String,
    /// The auth code associated with credit card payment methods.
    #[serde(rename = "authCode")]
    pub auth_code: String,
    /// The masked account number into which funds were deposited.
    #[serde(rename = "maskedPan")]
    pub masked_pan: String,
    /// That payment is still pending.
    #[serde(rename = "pending")]
    pub pending: bool,
    /// That payment is approved.
    #[serde(rename = "approved")]
    pub approved: bool,
    /// A response description from the disbursement payment platform, in any.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The amount disbursed in floating point format.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The currency formatted form of amount.
    #[serde(rename = "amountFormatted")]
    pub amount_formatted: String,

}

/// Models information about merchant deposits during a statement period.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StatementDeposit {
    /// The line item id.
    #[serde(rename = "id")]
    pub id: String,
    /// The number of transactions in the batch for which funds were deposited.
    #[serde(rename = "transactionCount")]
    pub transaction_count: i64,
    /// The batch id associated with the deposit.
    #[serde(rename = "batchId")]
    pub batch_id: String,
    /// The prepaid fees associated with the batch.
    #[serde(rename = "feesPaid")]
    pub fees_paid: f64,
    /// The currency formatted form of prepaid fees.
    #[serde(rename = "feesPaidFormatted")]
    pub fees_paid_formatted: String,
    /// The net deposit released to the merchant.
    #[serde(rename = "netDeposit")]
    pub net_deposit: f64,
    /// The currency formatted net deposit released to the merchant.
    #[serde(rename = "netDepositFormatted")]
    pub net_deposit_formatted: String,
    /// The total sales in the batch.
    #[serde(rename = "totalSales")]
    pub total_sales: f64,
    /// The currency formatted total sales in the batch.
    #[serde(rename = "totalSalesFormatted")]
    pub total_sales_formatted: String,
    /// The total refunds in the batch.
    #[serde(rename = "totalRefunds")]
    pub total_refunds: f64,
    /// The currency formatted total refunds in the batch.
    #[serde(rename = "totalRefundsFormatted")]
    pub total_refunds_formatted: String,

}

/// Models a request to retrieve detailed merchant invoice information.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PartnerCommissionBreakdownRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The invoice or statement id.
    #[serde(rename = "statementId")]
    pub statement_id: String,

}

/// Models detailed information about how partner commissions were calculated for a
/// statement.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PartnerCommissionBreakdownResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The invoice (statement id) for which the commissions were calculated.
    #[serde(rename = "invoiceId")]
    pub invoice_id: String,
    /// The partner name.
    #[serde(rename = "partnerName")]
    pub partner_name: String,
    /// The partner statement id.
    #[serde(rename = "partnerStatementId")]
    pub partner_statement_id: String,
    /// The partner statement date.
    #[serde(rename = "partnerStatementDate")]
    pub partner_statement_date: DateTime<Utc>,
    /// The merchant id.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    /// The merchant's corporate name.
    #[serde(rename = "merchantCompanyName")]
    pub merchant_company_name: String,
    /// The merchant's dba name.
    #[serde(rename = "merchantDbaName")]
    pub merchant_dba_name: String,
    /// The grand total.
    #[serde(rename = "grandTotal")]
    pub grand_total: f64,
    /// The currency formatted grand total.
    #[serde(rename = "grandTotalFormatted")]
    pub grand_total_formatted: String,
    /// The total fees.
    #[serde(rename = "totalFees")]
    pub total_fees: f64,
    /// The currency formatted total fees.
    #[serde(rename = "totalFeesFormatted")]
    pub total_fees_formatted: String,
    /// The total due the partner for this merchant statement.
    #[serde(rename = "totalDue")]
    pub total_due: f64,
    /// The currency formatted total due the partner for this merchant statement.
    #[serde(rename = "totalDueFormatted")]
    pub total_due_formatted: String,
    /// The total volume during the statement period.
    #[serde(rename = "totalVolume")]
    pub total_volume: f64,
    /// The currency formatted total volume during the statement period.
    #[serde(rename = "totalVolumeFormatted")]
    pub total_volume_formatted: String,
    /// The total number of transactions processed during the statement period.
    #[serde(rename = "totalTransactions")]
    pub total_transactions: i64,
    /// The residual earned by the partner.
    #[serde(rename = "partnerResidual")]
    pub partner_residual: f64,
    /// The currency formatted residual earned by the partner.
    #[serde(rename = "partnerResidualFormatted")]
    pub partner_residual_formatted: String,
    /// The total interchange charged during the statement period.
    #[serde(rename = "interchange")]
    pub interchange: f64,
    /// The currency formatted total interchange charged during the statement period.
    #[serde(rename = "interchangeFormatted")]
    pub interchange_formatted: String,
    /// The total assessments charged during the statement period.
    #[serde(rename = "assessments")]
    pub assessments: f64,
    /// The currency formatted assessments charged during the statement period.
    #[serde(rename = "assessmentsFormatted")]
    pub assessments_formatted: String,
    /// The total of passthrough costs.
    #[serde(rename = "totalPassthrough")]
    pub total_passthrough: f64,
    /// The currency formatted total of passthrough costs.
    #[serde(rename = "totalPassthroughFormatted")]
    pub total_passthrough_formatted: String,
    /// The total of non passthrough costs.
    #[serde(rename = "totalNonPassthrough")]
    pub total_non_passthrough: f64,
    /// The currency formatted total of non passthrough costs.
    #[serde(rename = "totalNonPassthroughFormatted")]
    pub total_non_passthrough_formatted: String,
    /// The total of all card brand fees.
    #[serde(rename = "totalCardBrandFees")]
    pub total_card_brand_fees: f64,
    /// The currency formatted total of all card brand fees.
    #[serde(rename = "totalCardBrandFeesFormatted")]
    pub total_card_brand_fees_formatted: String,
    /// The total buy rate.
    #[serde(rename = "totalBuyRate")]
    pub total_buy_rate: f64,
    /// The currency formatted total buy rate.
    #[serde(rename = "totalBuyRateFormatted")]
    pub total_buy_rate_formatted: String,
    /// The total buy rate before passthrough costs.
    #[serde(rename = "buyRateBeforePassthrough")]
    pub buy_rate_before_passthrough: f64,
    /// The currency formatted total buy rate before passthrough costs.
    #[serde(rename = "buyRateBeforePassthroughFormatted")]
    pub buy_rate_before_passthrough_formatted: String,
    /// The net markup split between BlockChyp and the partner.
    #[serde(rename = "netMarkup")]
    pub net_markup: f64,
    /// The currency formatted net markup split between BlockChyp and the partner.
    #[serde(rename = "netMarkupFormatted")]
    pub net_markup_formatted: String,
    /// The partner's total share of non passthrough hard costs.
    #[serde(rename = "partnerNonPassthroughShare")]
    pub partner_non_passthrough_share: f64,
    /// The currency formatted partner's total share of non passthrough hard costs.
    #[serde(rename = "partnerNonPassthroughShareFormatted")]
    pub partner_non_passthrough_share_formatted: String,
    /// The total of chargeback fees assessed during the statement period.
    #[serde(rename = "chargebackFees")]
    pub chargeback_fees: f64,
    /// The currency formatted total of chargeback fees assessed during the statement
/// period.
    #[serde(rename = "chargebackFeesFormatted")]
    pub chargeback_fees_formatted: String,
    /// The total number of chargebacks during the period.
    #[serde(rename = "chargebackCount")]
    pub chargeback_count: i64,
    /// The partner's share of markup.
    #[serde(rename = "partnerPercentage")]
    pub partner_percentage: f64,
    /// The currency formatted partner's share of markup.
    #[serde(rename = "partnerPercentageFormatted")]
    pub partner_percentage_formatted: String,
    /// The list of line items documenting how the total buy rate was calculated.
    #[serde(rename = "buyRateLineItems")]
    pub buy_rate_line_items: Option<Vec<BuyRateLineItem>>,
    /// The list of detail lines describing how revenue was calculated and collected by the
/// sponsor bank.
    #[serde(rename = "revenueDetails")]
    pub revenue_details: Option<Vec<AggregateBillingLineItem>>,
    /// The nested list of costs levied by the card brands, grouped by card brand and type.
    #[serde(rename = "cardBrandCostDetails")]
    pub card_brand_cost_details: Option<Vec<AggregateBillingLineItem>>,

}

/// Models a request to generate merchant api credentials.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantCredentialGenerationRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The merchant id.
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    /// Protects the credentials from deletion.
    #[serde(rename = "deleteProtected")]
    pub delete_protected: bool,
    /// An optional array of role codes that will be assigned to the credentials.
    #[serde(rename = "roles")]
    pub roles: Option<Vec<String>>,
    /// Free form description of the purpose or intent behind the credentials.
    #[serde(rename = "notes")]
    pub notes: String,
    /// Type of credentials to generate, either API or TOKENIZING. Defaults to API.
    #[serde(rename = "credentialType")]
    pub credential_type: String,

}

/// Merchant api credential data.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantCredentialGenerationResponse {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,
    /// The merchant api key.
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// The merchant bearer token.
    #[serde(rename = "bearerToken")]
    pub bearer_token: String,
    /// The merchant signing key.
    #[serde(rename = "signingKey")]
    pub signing_key: String,
    /// The tokenizing key.
    #[serde(rename = "tokenizingKey")]
    pub tokenizing_key: String,

}

/// Models a single buy rate calculation line item.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BuyRateLineItem {
    /// Provides a basic description of the line item.
    #[serde(rename = "description")]
    pub description: String,
    /// The volume related to this line item.
    #[serde(rename = "volume")]
    pub volume: f64,
    /// The currency formatted volume related to this line item.
    #[serde(rename = "volumeFormatted")]
    pub volume_formatted: String,
    /// The rate on volume charged for this line item.
    #[serde(rename = "volumeRate")]
    pub volume_rate: f64,
    /// The string formatted rate on volume charged for this line item.
    #[serde(rename = "volumeRateFormatted")]
    pub volume_rate_formatted: String,
    /// The count associated with this line item.
    #[serde(rename = "count")]
    pub count: i64,
    /// The rate per item charged for this line item.
    #[serde(rename = "countRate")]
    pub count_rate: f64,
    /// The string formatted rate per item charged for this line item.
    #[serde(rename = "countRateFormatted")]
    pub count_rate_formatted: String,
    /// Provides a narrative description of the rate.
    #[serde(rename = "rateSummary")]
    pub rate_summary: String,
    /// The total amount for the line item.
    #[serde(rename = "total")]
    pub total: f64,
    /// The string formatted total for the line item.
    #[serde(rename = "totalFormatted")]
    pub total_formatted: String,

}

/// Models low level aggregated and nested data line items.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AggregateBillingLineItem {
    /// The line item identifier.
    #[serde(rename = "id")]
    pub id: String,
    /// Provides a basic description of the line item.
    #[serde(rename = "description")]
    pub description: String,
    /// That a line item has nested information.
    #[serde(rename = "expandable")]
    pub expandable: bool,
    /// The total is a negative number.
    #[serde(rename = "negative")]
    pub negative: bool,
    /// The total for the line item.
    #[serde(rename = "total")]
    pub total: f64,
    /// The currency formatted total for the line item.
    #[serde(rename = "totalFormatted")]
    pub total_formatted: String,
    /// The range of count based fees charged for the given line item.
    #[serde(rename = "perTranFeeRange")]
    pub per_tran_fee_range: Option<AggregateBillingLineItemStats>,
    /// The range of percentage based fees charged for the given line item.
    #[serde(rename = "transactionPercentageRange")]
    pub transaction_percentage_range: Option<AggregateBillingLineItemStats>,
    /// Encapsulated drill down or detail lines.
    #[serde(rename = "detailLines")]
    pub detail_lines: Option<Vec<AggregateBillingLineItem>>,

}

/// Models statistics for low level aggregation line items.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AggregateBillingLineItemStats {
    /// The min value in the set.
    #[serde(rename = "min")]
    pub min: String,
    /// The avg value in the set.
    #[serde(rename = "avg")]
    pub avg: String,
    /// The max value in the set.
    #[serde(rename = "max")]
    pub max: String,

}

/// Models an individual with 25% or more ownership interest in a company.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Owner {
    /// The first name of the owner.
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// The last name of the owner.
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// The job title of the owner.
    #[serde(rename = "jobTitle")]
    pub job_title: String,
    /// The tax identification number (SSN) of the owner.
    #[serde(rename = "taxIdNumber")]
    pub tax_id_number: String,
    /// The phone number of the owner.
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    /// The date of birth of the owner in mm/dd/yyyy format.
    #[serde(rename = "dob")]
    pub dob: String,
    /// The percentage of ownership.
    #[serde(rename = "ownership")]
    pub ownership: String,
    /// The address of the owner.
    #[serde(rename = "address")]
    pub address: Address,
    /// The email address of the owner.
    #[serde(rename = "email")]
    pub email: String,
    /// A single line representation of the owner's address.
    #[serde(rename = "singleLineAddress")]
    pub single_line_address: String,
    /// The type of entity this owner represents.
    #[serde(rename = "entityType")]
    pub entity_type: String,
    /// The driver's license number of the owner.
    #[serde(rename = "dlNumber")]
    pub dl_number: String,
    /// The state that issued the owner's driver's license.
    #[serde(rename = "dlStateOrProvince")]
    pub dl_state_or_province: String,
    /// The expiration date of the owner's driver's license.
    #[serde(rename = "dlExpiration")]
    pub dl_expiration: String,

}

/// Models a bank account associated with an application.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ApplicationAccount {
    /// The name of the bank account.
    #[serde(rename = "name")]
    pub name: String,
    /// The name of the bank.
    #[serde(rename = "bank")]
    pub bank: String,
    /// The name of the account holder.
    #[serde(rename = "accountHolderName")]
    pub account_holder_name: String,
    /// The routing number of the bank.
    #[serde(rename = "routingNumber")]
    pub routing_number: String,
    /// The account number.
    #[serde(rename = "accountNumber")]
    pub account_number: String,

}

/// Models a merchant application form to add a merchant account.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MerchantApplication {
    /// The invite code for the merchant.
    #[serde(rename = "inviteCode")]
    pub invite_code: String,
    /// The business name your customers know you by (DBA Name).
    #[serde(rename = "dbaName")]
    pub dba_name: String,
    /// The name of the legal entity you file your taxes under.
    #[serde(rename = "corporateName")]
    pub corporate_name: String,
    /// The business website.
    #[serde(rename = "webSite")]
    pub web_site: String,
    /// The business tax identification number (EIN).
    #[serde(rename = "taxIdNumber")]
    pub tax_id_number: String,
    /// The type of business entity.
    #[serde(rename = "entityType")]
    pub entity_type: String,
    /// The state where the business is incorporated.
    #[serde(rename = "stateOfIncorporation")]
    pub state_of_incorporation: String,
    /// The primary type of business (e.g., Retail, Service, etc.).
    #[serde(rename = "merchantType")]
    pub merchant_type: String,
    /// A short description of the products and services sold.
    #[serde(rename = "businessDescription")]
    pub business_description: String,
    /// The number of years the business has been operating.
    #[serde(rename = "yearsInBusiness")]
    pub years_in_business: String,
    /// The business telephone number.
    #[serde(rename = "businessPhoneNumber")]
    pub business_phone_number: String,
    /// The physical address of the business.
    #[serde(rename = "physicalAddress")]
    pub physical_address: Address,
    /// The mailing address of the business.
    #[serde(rename = "mailingAddress")]
    pub mailing_address: Address,
    /// The first name of the primary contact.
    #[serde(rename = "contactFirstName")]
    pub contact_first_name: String,
    /// The last name of the primary contact.
    #[serde(rename = "contactLastName")]
    pub contact_last_name: String,
    /// The phone number of the primary contact.
    #[serde(rename = "contactPhoneNumber")]
    pub contact_phone_number: String,
    /// The email address of the primary contact.
    #[serde(rename = "contactEmail")]
    pub contact_email: String,
    /// The job title of the primary contact.
    #[serde(rename = "contactTitle")]
    pub contact_title: String,
    /// The tax identification number (SSN) of the primary contact.
    #[serde(rename = "contactTaxIdNumber")]
    pub contact_tax_id_number: String,
    /// The date of birth of the primary contact.
    #[serde(rename = "contactDOB")]
    pub contact_dob: String,
    /// The driver's license number of the primary contact.
    #[serde(rename = "contactDlNumber")]
    pub contact_dl_number: String,
    /// The state that issued the primary contact's driver's license.
    #[serde(rename = "contactDlStateOrProvince")]
    pub contact_dl_state_or_province: String,
    /// The expiration date of the primary contact's driver's license.
    #[serde(rename = "contactDlExpiration")]
    pub contact_dl_expiration: String,
    /// The home address of the primary contact.
    #[serde(rename = "contactHomeAddress")]
    pub contact_home_address: Address,
    /// The role of the primary contact in the business.
    #[serde(rename = "contactRole")]
    pub contact_role: String,
    /// List of individuals with 25% or more ownership in the company.
    #[serde(rename = "owners")]
    pub owners: Option<Vec<Owner>>,
    /// The bank account information for the business.
    #[serde(rename = "manualAccount")]
    pub manual_account: ApplicationAccount,
    /// The average transaction amount.
    #[serde(rename = "averageTransaction")]
    pub average_transaction: String,
    /// The highest expected transaction amount.
    #[serde(rename = "highTransaction")]
    pub high_transaction: String,
    /// The average monthly transaction volume.
    #[serde(rename = "averageMonth")]
    pub average_month: String,
    /// The highest expected monthly transaction volume.
    #[serde(rename = "highMonth")]
    pub high_month: String,
    /// The refund policy of the business.
    #[serde(rename = "refundPolicy")]
    pub refund_policy: String,
    /// The number of days after purchase that refunds can be issued.
    #[serde(rename = "refundDays")]
    pub refund_days: String,
    /// The time zone of the business.
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    /// The time when the daily batch should close.
    #[serde(rename = "batchCloseTime")]
    pub batch_close_time: String,
    /// Indicates if the business has multiple locations.
    #[serde(rename = "multipleLocations")]
    pub multiple_locations: String,
    /// The name of this specific business location.
    #[serde(rename = "locationName")]
    pub location_name: String,
    /// The store number for this location.
    #[serde(rename = "storeNumber")]
    pub store_number: String,
    /// Indicates if the business wants to accept EBT cards.
    #[serde(rename = "ebtRequested")]
    pub ebt_requested: String,
    /// The FNS number issued by the USDA for EBT processing.
    #[serde(rename = "fnsNumber")]
    pub fns_number: String,
    /// Indicates if the business plans to accept payments through a website.
    #[serde(rename = "ecommerce")]
    pub ecommerce: String,
    /// Indicates if suppliers ship products directly to customers.
    #[serde(rename = "dropShipping")]
    pub drop_shipping: bool,
    /// The percentage of transactions that will be chip or swipe.
    #[serde(rename = "cardPresentPercentage")]
    pub card_present_percentage: String,
    /// The percentage of transactions that will be phone orders.
    #[serde(rename = "phoneOrderPercentage")]
    pub phone_order_percentage: String,
    /// The percentage of transactions that will be e-commerce.
    #[serde(rename = "ecomPercentage")]
    pub ecom_percentage: String,
    /// The number of days before shipment that customers are charged.
    #[serde(rename = "billBeforeShipmentDays")]
    pub bill_before_shipment_days: String,
    /// Indicates if the business plans to process recurring payments.
    #[serde(rename = "subscriptionsSupported")]
    pub subscriptions_supported: String,
    /// The frequency of recurring payments (if applicable).
    #[serde(rename = "subscriptionFrequency")]
    pub subscription_frequency: String,
    /// The full legal name of the person signing the application.
    #[serde(rename = "signerName")]
    pub signer_name: String,

}

/// Models a merchant application submission request to add a new merchant account.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubmitApplicationRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The invite code for the merchant.
    #[serde(rename = "inviteCode")]
    pub invite_code: String,
    /// The business name your customers know you by (DBA Name).
    #[serde(rename = "dbaName")]
    pub dba_name: String,
    /// The name of the legal entity you file your taxes under.
    #[serde(rename = "corporateName")]
    pub corporate_name: String,
    /// The business website.
    #[serde(rename = "webSite")]
    pub web_site: String,
    /// The business tax identification number (EIN).
    #[serde(rename = "taxIdNumber")]
    pub tax_id_number: String,
    /// The type of business entity.
    #[serde(rename = "entityType")]
    pub entity_type: String,
    /// The state where the business is incorporated.
    #[serde(rename = "stateOfIncorporation")]
    pub state_of_incorporation: String,
    /// The primary type of business (e.g., Retail, Service, etc.).
    #[serde(rename = "merchantType")]
    pub merchant_type: String,
    /// A short description of the products and services sold.
    #[serde(rename = "businessDescription")]
    pub business_description: String,
    /// The number of years the business has been operating.
    #[serde(rename = "yearsInBusiness")]
    pub years_in_business: String,
    /// The business telephone number.
    #[serde(rename = "businessPhoneNumber")]
    pub business_phone_number: String,
    /// The physical address of the business.
    #[serde(rename = "physicalAddress")]
    pub physical_address: Address,
    /// The mailing address of the business.
    #[serde(rename = "mailingAddress")]
    pub mailing_address: Address,
    /// The first name of the primary contact.
    #[serde(rename = "contactFirstName")]
    pub contact_first_name: String,
    /// The last name of the primary contact.
    #[serde(rename = "contactLastName")]
    pub contact_last_name: String,
    /// The phone number of the primary contact.
    #[serde(rename = "contactPhoneNumber")]
    pub contact_phone_number: String,
    /// The email address of the primary contact.
    #[serde(rename = "contactEmail")]
    pub contact_email: String,
    /// The job title of the primary contact.
    #[serde(rename = "contactTitle")]
    pub contact_title: String,
    /// The tax identification number (SSN) of the primary contact.
    #[serde(rename = "contactTaxIdNumber")]
    pub contact_tax_id_number: String,
    /// The date of birth of the primary contact.
    #[serde(rename = "contactDOB")]
    pub contact_dob: String,
    /// The driver's license number of the primary contact.
    #[serde(rename = "contactDlNumber")]
    pub contact_dl_number: String,
    /// The state that issued the primary contact's driver's license.
    #[serde(rename = "contactDlStateOrProvince")]
    pub contact_dl_state_or_province: String,
    /// The expiration date of the primary contact's driver's license.
    #[serde(rename = "contactDlExpiration")]
    pub contact_dl_expiration: String,
    /// The home address of the primary contact.
    #[serde(rename = "contactHomeAddress")]
    pub contact_home_address: Address,
    /// The role of the primary contact in the business.
    #[serde(rename = "contactRole")]
    pub contact_role: String,
    /// List of individuals with 25% or more ownership in the company.
    #[serde(rename = "owners")]
    pub owners: Option<Vec<Owner>>,
    /// The bank account information for the business.
    #[serde(rename = "manualAccount")]
    pub manual_account: ApplicationAccount,
    /// The average transaction amount.
    #[serde(rename = "averageTransaction")]
    pub average_transaction: String,
    /// The highest expected transaction amount.
    #[serde(rename = "highTransaction")]
    pub high_transaction: String,
    /// The average monthly transaction volume.
    #[serde(rename = "averageMonth")]
    pub average_month: String,
    /// The highest expected monthly transaction volume.
    #[serde(rename = "highMonth")]
    pub high_month: String,
    /// The refund policy of the business.
    #[serde(rename = "refundPolicy")]
    pub refund_policy: String,
    /// The number of days after purchase that refunds can be issued.
    #[serde(rename = "refundDays")]
    pub refund_days: String,
    /// The time zone of the business.
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    /// The time when the daily batch should close.
    #[serde(rename = "batchCloseTime")]
    pub batch_close_time: String,
    /// Indicates if the business has multiple locations.
    #[serde(rename = "multipleLocations")]
    pub multiple_locations: String,
    /// The name of this specific business location.
    #[serde(rename = "locationName")]
    pub location_name: String,
    /// The store number for this location.
    #[serde(rename = "storeNumber")]
    pub store_number: String,
    /// Indicates if the business wants to accept EBT cards.
    #[serde(rename = "ebtRequested")]
    pub ebt_requested: String,
    /// The FNS number issued by the USDA for EBT processing.
    #[serde(rename = "fnsNumber")]
    pub fns_number: String,
    /// Indicates if the business plans to accept payments through a website.
    #[serde(rename = "ecommerce")]
    pub ecommerce: String,
    /// Indicates if suppliers ship products directly to customers.
    #[serde(rename = "dropShipping")]
    pub drop_shipping: bool,
    /// The percentage of transactions that will be chip or swipe.
    #[serde(rename = "cardPresentPercentage")]
    pub card_present_percentage: String,
    /// The percentage of transactions that will be phone orders.
    #[serde(rename = "phoneOrderPercentage")]
    pub phone_order_percentage: String,
    /// The percentage of transactions that will be e-commerce.
    #[serde(rename = "ecomPercentage")]
    pub ecom_percentage: String,
    /// The number of days before shipment that customers are charged.
    #[serde(rename = "billBeforeShipmentDays")]
    pub bill_before_shipment_days: String,
    /// Indicates if the business plans to process recurring payments.
    #[serde(rename = "subscriptionsSupported")]
    pub subscriptions_supported: String,
    /// The frequency of recurring payments (if applicable).
    #[serde(rename = "subscriptionFrequency")]
    pub subscription_frequency: String,
    /// The full legal name of the person signing the application.
    #[serde(rename = "signerName")]
    pub signer_name: String,

}



/// A request for customer signature data.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalCaptureSignatureRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: CaptureSignatureRequest,
}

/// Information needed to test connectivity with a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalPingRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: PingRequest,
}

/// Information needed to retrieve location information for a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalLocateRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: LocateRequest,
}

/// A message to be displayed on the terminal screen.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalMessageRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: MessageRequest,
}

/// A simple yes no prompt request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalBooleanPromptRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: BooleanPromptRequest,
}

/// A text prompt request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalTextPromptRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: TextPromptRequest,
}

/// An authorization request for a charge, preauth, or reverse transaction.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalAuthorizationRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: AuthorizationRequest,
}

/// Retrieves card metadata.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalCardMetadataRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: CardMetadataRequest,
}

/// A request for the remaining balance on a payment type.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalBalanceRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: BalanceRequest,
}

/// A refund request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalRefundRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: RefundRequest,
}

/// The information needed to enroll a new payment method in the token vault.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalEnrollRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: EnrollRequest,
}

/// The information needed to enroll a new payment method in the token vault.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalClearTerminalRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: ClearTerminalRequest,
}

/// The information needed to activate or recharge a gift card.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalGiftActivateRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: GiftActivateRequest,
}

/// The fields needed for custom Terms and Conditions prompts.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalTermsAndConditionsRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: TermsAndConditionsRequest,
}

/// A signature capture response for Terms and Conditions.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalTermsAndConditionsResponse {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: TermsAndConditionsResponse,
}

/// Used to start or update a transaction line item display on a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalTransactionDisplayRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: TransactionDisplayRequest,
}

/// A request for the status of a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalTerminalStatusRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: TerminalStatusRequest,
}

/// Returns a list of queued transactions on a terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalListQueuedTransactionsRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: ListQueuedTransactionsRequest,
}

/// Deletes one or all transactions from a terminal queue.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalDeleteQueuedTransactionRequest {
    #[serde(flatten)]
    pub api_credentials: APICredentials,
    #[serde(rename = "request")]
    pub request: DeleteQueuedTransactionRequest,
}



/// Fields which should be returned with standard requests.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AbstractAcknowledgement {
    /// Whether or not the request succeeded.
    #[serde(rename = "success")]
    pub success: bool,
    /// The error, if an error occurred.
    #[serde(rename = "error")]
    pub error: String,
    /// A narrative description of the transaction result.
    #[serde(rename = "responseDescription")]
    pub response_description: String,

}

/// A reference to a terminal name.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalReference {
    /// The name of the target payment terminal.
    #[serde(rename = "terminalName")]
    pub terminal_name: String,
    /// Forces the terminal cloud connection to be reset while a transactions is in flight.
/// This is a diagnostic settings that can be used only for test transactions.
    #[serde(rename = "resetConnection")]
    pub reset_connection: bool,

}

/// Customer signature data.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SignatureResponse {
    /// The hex encoded signature data.
    #[serde(rename = "sigFile")]
    pub sig_file: String,

}

/// A request for customer signature data.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SignatureRequest {
    /// A location on the filesystem which a customer signature should be written to.
    #[serde(rename = "sigFile")]
    pub sig_file: String,
    /// The image format to be used for returning signatures.
    #[serde(rename = "sigFormat")]
    pub sig_format: SignatureFormat,
    /// The width that the signature image should be scaled to, preserving the aspect ratio. If
/// not provided, the signature is returned in the terminal's max resolution.
    #[serde(rename = "sigWidth")]
    pub sig_width: i32,
    /// Whether or not signature prompt should be skipped on the terminal. The terminal will
/// indicate whether or not a signature is required by the card in the receipt suggestions
/// response.
    #[serde(rename = "disableSignature")]
    pub disable_signature: bool,

}

/// Response fields for an approved transaction.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {
    /// That the transaction was approved.
    #[serde(rename = "approved")]
    pub approved: bool,
    /// The auth code from the payment network.
    #[serde(rename = "authCode")]
    pub auth_code: String,
    /// The code returned by the terminal or the card issuer to indicate the disposition of the
/// message.
    #[serde(rename = "authResponseCode")]
    pub auth_response_code: String,

}

/// Models a low level request with a timeout and test flag.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TimeoutRequest {
    /// The request timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// Whether or not to route transaction to the test gateway.
    #[serde(rename = "test")]
    pub test: bool,

}

/// Core request fields for a transaction.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CoreRequest {
    /// A user-assigned reference that can be used to recall or reverse transactions.
    #[serde(rename = "transactionRef")]
    pub transaction_ref: String,
    /// That the transaction reference was autogenerated and should be ignored for the
/// purposes of duplicate detection.
    #[serde(rename = "autogeneratedRef")]
    pub autogenerated_ref: bool,
    /// Defers the response to the transaction and returns immediately. Callers should
/// retrive the transaction result using the Transaction Status API.
    #[serde(rename = "async")]
    pub async_yo: bool,
    /// Adds the transaction to the queue and returns immediately. Callers should retrive the
/// transaction result using the Transaction Status API.
    #[serde(rename = "queue")]
    pub queue: bool,
    /// Whether or not the request should block until all cards have been removed from the card
/// reader.
    #[serde(rename = "waitForRemovedCard")]
    pub wait_for_removed_card: bool,
    /// Override any in-progress transactions.
    #[serde(rename = "force")]
    pub force: bool,
    /// An identifier from an external point of sale system.
    #[serde(rename = "orderRef")]
    pub order_ref: String,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount")]
    pub destination_account: String,
    /// Can include a code used to trigger simulated conditions for the purposes of testing and
/// certification. Valid for test merchant accounts only.
    #[serde(rename = "testCase")]
    pub test_case: String,

}

/// Response details about a payment method.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodResponse {
    /// The payment token, if the payment was enrolled in the vault.
    #[serde(rename = "token")]
    pub token: String,
    /// The entry method for the transaction (CHIP, MSR, KEYED, etc).
    #[serde(rename = "entryMethod")]
    pub entry_method: String,
    /// The card brand (VISA, MC, AMEX, DEBIT, etc).
    #[serde(rename = "paymentType")]
    pub payment_type: String,
    /// Provides network level detail on how a transaction was routed, especially for debit
/// transactions.
    #[serde(rename = "network")]
    pub network: String,
    /// Identifies the card association based on bin number. Used primarily used to indicate
/// the major logo on a card, even when debit transactions are routed on a different
/// network.
    #[serde(rename = "logo")]
    pub logo: String,
    /// The masked primary account number.
    #[serde(rename = "maskedPan")]
    pub masked_pan: String,
    /// The BlockChyp public key if the user presented a BlockChyp payment card.
    #[serde(rename = "publicKey")]
    pub public_key: String,
    /// That the transaction did something that would put the system in PCI scope.
    #[serde(rename = "ScopeAlert")]
    pub scope_alert: bool,
    /// The cardholder name.
    #[serde(rename = "cardHolder")]
    pub card_holder: String,
    /// The card expiration month in MM format.
    #[serde(rename = "expMonth")]
    pub exp_month: String,
    /// The card expiration year in YY format.
    #[serde(rename = "expYear")]
    pub exp_year: String,
    /// The card postal code.
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    /// The card address.
    #[serde(rename = "address")]
    pub address: String,
    /// The card country.
    #[serde(rename = "country")]
    pub country: String,
    /// Address verification results if address information was submitted.
    #[serde(rename = "avsResponse")]
    pub avs_response: AVSResponse,
    /// The CVV verification result if CVV was submitted.
    #[serde(rename = "cvvResponse")]
    pub cvv_response: String,
    /// Suggested receipt fields.
    #[serde(rename = "receiptSuggestions")]
    pub receipt_suggestions: ReceiptSuggestions,
    /// Customer data, if any. Preserved for reverse compatibility.
    #[serde(rename = "customer")]
    pub customer: Option<Customer>,
    /// Customer data, if any.
    #[serde(rename = "customers")]
    pub customers: Customer,

}

/// Response details for a cryptocurrency transaction.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CryptocurrencyResponse {
    /// That the transaction has met the standard criteria for confirmation on the network.
/// (For example, 6 confirmations for level one bitcoin.)
    #[serde(rename = "confirmed")]
    pub confirmed: bool,
    /// The amount submitted to the blockchain.
    #[serde(rename = "cryptoAuthorizedAmount")]
    pub crypto_authorized_amount: String,
    /// The network level fee assessed for the transaction denominated in cryptocurrency.
/// This fee goes to channel operators and crypto miners, not BlockChyp.
    #[serde(rename = "cryptoNetworkFee")]
    pub crypto_network_fee: String,
    /// The three letter cryptocurrency code used for the transactions.
    #[serde(rename = "cryptocurrency")]
    pub cryptocurrency: String,
    /// Whether or not the transaction was processed on the level one or level two network.
    #[serde(rename = "cryptoNetwork")]
    pub crypto_network: String,
    /// The address on the crypto network the transaction was sent to.
    #[serde(rename = "cryptoReceiveAddress")]
    pub crypto_receive_address: String,
    /// Hash or other identifier that identifies the block on the cryptocurrency network, if
/// available or relevant.
    #[serde(rename = "cryptoBlock")]
    pub crypto_block: String,
    /// Hash or other transaction identifier that identifies the transaction on the
/// cryptocurrency network, if available or relevant.
    #[serde(rename = "cryptoTransactionId")]
    pub crypto_transaction_id: String,
    /// The payment request URI used for the transaction, if available.
    #[serde(rename = "cryptoPaymentRequest")]
    pub crypto_payment_request: String,
    /// Used for additional status information related to crypto transactions.
    #[serde(rename = "cryptoStatus")]
    pub crypto_status: String,

}

/// Response details about tender amounts.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentAmounts {
    /// Whether or not the transaction was approved for a partial amount.
    #[serde(rename = "partialAuth")]
    pub partial_auth: bool,
    /// Whether or not an alternate currency was used.
    #[serde(rename = "altCurrency")]
    pub alt_currency: bool,
    /// Whether or not a request was settled on an FSA card.
    #[serde(rename = "fsaAuth")]
    pub fsa_auth: bool,
    /// The currency code used for the transaction.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The requested amount.
    #[serde(rename = "requestedAmount")]
    pub requested_amount: String,
    /// The authorized amount. May not match the requested amount in the event of a partial
/// auth.
    #[serde(rename = "authorizedAmount")]
    pub authorized_amount: String,
    /// The remaining balance on the payment method.
    #[serde(rename = "remainingBalance")]
    pub remaining_balance: String,
    /// The tip amount.
    #[serde(rename = "tipAmount")]
    pub tip_amount: String,
    /// The tax amount.
    #[serde(rename = "taxAmount")]
    pub tax_amount: String,
    /// The cash back amount the customer requested during the transaction.
    #[serde(rename = "requestedCashBackAmount")]
    pub requested_cash_back_amount: String,
    /// The amount of cash back authorized by the gateway. This amount will be the entire amount
/// requested, or zero.
    #[serde(rename = "authorizedCashBackAmount")]
    pub authorized_cash_back_amount: String,

}

/// Request details about a payment method.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    /// The payment token to be used for this transaction. This should be used for recurring
/// transactions.
    #[serde(rename = "token")]
    pub token: String,
    /// Track 1 magnetic stripe data.
    #[serde(rename = "track1")]
    pub track_1: String,
    /// Track 2 magnetic stripe data.
    #[serde(rename = "track2")]
    pub track_2: String,
    /// The primary account number. We recommend using the terminal or e-commerce
/// tokenization libraries instead of passing account numbers in directly, as this would
/// put your application in PCI scope.
    #[serde(rename = "pan")]
    pub pan: String,
    /// The ACH routing number for ACH transactions.
    #[serde(rename = "routingNumber")]
    pub routing_number: String,
    /// The cardholder name. Only required if the request includes a primary account number or
/// track data.
    #[serde(rename = "cardholderName")]
    pub cardholder_name: String,
    /// The card expiration month for use with PAN based transactions.
    #[serde(rename = "expMonth")]
    pub exp_month: String,
    /// The card expiration year for use with PAN based transactions.
    #[serde(rename = "expYear")]
    pub exp_year: String,
    /// The card CVV for use with PAN based transactions.
    #[serde(rename = "cvv")]
    pub cvv: String,
    /// The cardholder address for use with address verification.
    #[serde(rename = "address")]
    pub address: String,
    /// The cardholder postal code for use with address verification.
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    /// The cardholder country.
    #[serde(rename = "country")]
    pub country: String,
    /// That the payment entry method is a manual keyed transaction. If this is true, no other
/// payment method will be accepted.
    #[serde(rename = "manualEntry")]
    pub manual_entry: bool,
    /// The key serial number used for DUKPT encryption.
    #[serde(rename = "ksn")]
    pub ksn: String,
    /// The encrypted pin block.
    #[serde(rename = "pinBlock")]
    pub pin_block: String,
    /// Designates categories of cards: credit, debit, EBT.
    #[serde(rename = "cardType")]
    pub card_type: CardType,
    /// Designates brands of payment methods: Visa, Discover, etc.
    #[serde(rename = "paymentType")]
    pub payment_type: String,

}

/// Request details about tender amounts.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RequestAmount {
    /// The transaction currency code.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// The requested amount.
    #[serde(rename = "amount")]
    pub amount: String,
    /// That the request is tax exempt. Only required for tax exempt level 2 processing.
    #[serde(rename = "taxExempt")]
    pub tax_exempt: bool,
    /// A flag to add a surcharge to the transaction to cover credit card fees, if permitted.
    #[serde(rename = "surcharge")]
    pub surcharge: bool,
    /// A flag that applies a discount to negate the surcharge for debit transactions or other
/// surcharge ineligible payment methods.
    #[serde(rename = "cashDiscount")]
    pub cash_discount: bool,

}

/// Request subtotals.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Subtotals {
    /// The tip amount.
    #[serde(rename = "tipAmount")]
    pub tip_amount: String,
    /// The tax amount.
    #[serde(rename = "taxAmount")]
    pub tax_amount: String,

}

/// A reference to a previous transaction.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PreviousTransaction {
    /// The ID of the previous transaction being referenced.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,

}

/// Core response fields for a transaction.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CoreResponse {
    /// The ID assigned to the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The ID assigned to the batch.
    #[serde(rename = "batchId")]
    pub batch_id: String,
    /// The transaction reference string assigned to the transaction request. If no
/// transaction ref was assiged on the request, then the gateway will randomly generate
/// one.
    #[serde(rename = "transactionRef")]
    pub transaction_ref: String,
    /// The type of transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The hash of the last tick block.
    #[serde(rename = "tickBlock")]
    pub tick_block: String,
    /// That the transaction was processed on the test gateway.
    #[serde(rename = "test")]
    pub test: bool,
    /// The settlement account for merchants with split settlements.
    #[serde(rename = "destinationAccount")]
    pub destination_account: String,
    /// The ECC signature of the response. Can be used to ensure that it was signed by the
/// terminal and detect man-in-the middle attacks.
    #[serde(rename = "sig")]
    pub sig: String,

}

pub fn clear_field<T>(data: T, field: &str) -> Result<T, Box<dyn std::error::Error>>
where 
   T: Serialize + DeserializeOwned + Clone,
{
   let mut json_value = serde_json::to_value(&data)?;

   if let Value::Object(ref mut map) = json_value {
      map.remove(field);
   }

   Ok(serde_json::from_value(json_value)?)
}

pub fn to_signature_request<T: Serialize>(request: &T) -> Result<SignatureRequest, Box<dyn std::error::Error>> {
    let request_json = serde_json::to_value(request)?;
    match serde_json::from_value::<SignatureRequest>(request_json) {
        Ok(signature_request) => Ok(signature_request),
        Err(err) => Err(Box::new(err)),
    }
}

pub fn to_signature_response<T: Serialize>(response: &T) -> Result<SignatureResponse, Box<dyn std::error::Error>> {
    let response_json = serde_json::to_value(response)?;
    match serde_json::from_value::<SignatureResponse>(response_json) {
        Ok(signature_response) => Ok(signature_response),
        Err(err) => Err(Box::new(err)),
    }
}

pub fn copy_from_signature_request<T, R>(from: &T, to: &mut R) -> Result<Option<R>, Box<dyn std::error::Error>>
where
    T: Serialize,
    R: DeserializeOwned + Serialize + Clone,
{
   let from_json = serde_json::to_value(from)?;
   let to_json = serde_json::to_value(to)?;
   let mut from_map = from_json.as_object().ok_or("Failed to convert from to object")?.clone();
   let mut to_map = to_json.as_object().ok_or("Failed to convert to to object")?.clone();
   let mut ok = false;

   for (key, value) in from_map.iter_mut() {
      if let Some(to_field) = to_map.get_mut(key) {
         *to_field = value.clone();
         ok = true;
      }
   }

   if ok {
      Ok(serde_json::from_value(to_json)?)
   } else {
      Ok(None)
   }
}
