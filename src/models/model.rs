#[derive(Debug, serde::Deserialize)]
pub struct ZainboxInfo {
    pub name: String,
    pub codeName: String,
    pub callbackUrl: String,
    pub isActive: bool,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateZainboxRequest {
    pub name: String,
    pub email_notification: String,
    pub callback_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_name_prefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_auto_internal_transfer: Option<bool>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SettlementAccount {
    account_number: String,
    bank_code: String,
    percentage: String,  // Stored as String to match PHP's strval conversion
}

impl SettlementAccount {
    pub fn new(account_number: String, bank_code: String, percentage: f64) -> Self {
        Self {
            account_number,
            bank_code,
            percentage: percentage.to_string(),
        }
    }
}
