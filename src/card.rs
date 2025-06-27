use crate::engine::Engine;
use crate::response::Response;
use crate::utils::filter::FilterUtil;
use serde_json::{json, Value};
use std::error::Error;

pub struct CardService {
    engine: Engine,
}

impl CardService {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    pub async fn initialize_new_payment(
        &self,
        amount: String,
        txn_ref: String,
        email_address: String,
        mobile_number: String,
        zainbox_code: String,
        callback_url: String,
    ) -> Result<Response, Box<dyn Error>> {
        let mut payload = json!({
            "amount": amount,
            "txnRef": txn_ref,
            "emailAddress": email_address,
            "mobileNumber": mobile_number,
            "zainboxCode": zainbox_code,
            "callbackUrl": callback_url
        });

        let http_response = self
            .engine
            .post("zainbox/card/initialize/payment", &payload)
            .await?;
        Ok(Response::new(http_response).await)
    }

    pub async fn verify_card_payment(&self, txn_ref: String) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "virtual-account/wallet/deposit/verify/{}",
                txn_ref
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }

    pub async fn verify_card_payment_v2(
        &self,
        txn_ref: String,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "virtual-account/wallet/deposit/verify/v2/{}",
                txn_ref
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }

    pub async fn reconcile_card_payment(
        &self,
        txn_ref: String,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "virtual-account/wallet/transaction/reconcile/card-payment?txnRef={}",
                txn_ref
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }

    pub async fn get_zainbox_card_payment_txn_history(
        &self,
        zainbox_code: String,
        count: Option<u32>,
        date_from: Option<String>,
        date_to: Option<String>,
        email: Option<String>,
        status: Option<String>,
        txn_ref: Option<String>,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "zainbox/card/transactions/{}?count={}&{}",
                zainbox_code,
                count.unwrap_or(10),
                FilterUtil::construct_filter_params(
                    date_from, date_to, email, status, txn_ref, None, None, None
                )
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }
}
