use crate::engine::Engine;
use crate::response::Response;
use serde_json::{json, Value};
use std::error::Error;

pub struct BankService {
    engine: Engine,
}

impl BankService {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    /**
     * Get the list of available banks.
     *
     * @return Response
     * @throws GuzzleException
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=get-bank-list
     */
    pub async fn get_bank_list(&self) -> Result<Response, Box<dyn Error>> {
        let http_response = self.engine.get("bank/list").await?;
        Ok(Response::new(http_response).await)
    }

    /**
     * Use the bankCode acquired from the get bank list to validate a bank account number.
     *
     * @param string $bankCode
     * @param string $accountNumber
     * @return Response
     * @throws GuzzleException
     * @link https://zainpay.ng/developers/api-endpoints?section=name-enquiry
     */
    pub async fn make_account_name_enquiry(
        &self,
        bank_code: String,
        account_number: String,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "bank/name-enquiry?bankCode={}&accountNumber={}",
                bank_code, account_number
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }

    /**
     * Fund transfers can be made in the following ways:
     * - Transferring money from one wallet to another
     * - Make a bank account transfer from your wallet.
     *
     * Zainpay infers your fund transfer type, so you don't have to specify it.
     * The charges for each form of transfer are different.
     * This charge can be obtained through your commercials.
     *
     * The amount in the JSON request should be converted to kobo decimalization.
     * It is expected that neither float nor double values will be utilized in this case.
     *
     * @param string $destinationAccountNumber
     * @param string $destinationBankCode
     * @param string $amount
     * @param string $sourceAccountNumber
     * @param string $sourceBankCode
     * @param string $zainboxCode
     * @param string $txnRef
     * @param string $narration
     * @param string|null $callbackUrl
     * @return Response
     * @throws GuzzleException
     * @link https://zainpay.ng/developers/api-endpoints?section=funds-transfer
     */
    pub async fn make_fund_transfer(
        &self,
        destination_account_number: String,
        destination_bank_code: String,
        amount: String,
        source_account_number: String,
        source_bank_code: String,
        zainbox_code: String,
        txn_ref: String,
        narration: String,
        callback_url: Option<String>,
    ) -> Result<Response, Box<dyn Error>> {
        let mut payload = json!({
            "destinationAccountNumber": destination_account_number,
            "destinationBankCode": destination_bank_code,
            "amount": amount,
            "sourceAccountNumber": source_account_number,
            "sourceBankCode": source_bank_code,
            "zainboxCode": zainbox_code,
            "txnRef": txn_ref,
            "narration": narration,
        });

        if let Some(callback_url) = callback_url {
            payload["callbackUrl"] = Value::from(callback_url);
        }

        let http_response = self.engine.post("bank/transfer", &payload).await?;
        Ok(Response::new(http_response).await)
    }

    pub async fn verify_transfer(&self, txn_ref: String) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "virtual-account/wallet/transaction/verify/{}",
                txn_ref
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }

    pub async fn verify_deposit(&self, txn_ref: String) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "virtual-account/wallet/transaction/deposit/verify/{}",
                txn_ref
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }

    pub async fn verify_deposit_v2(&self, txn_ref: String) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "virtual-account/wallet/transaction/deposit/verify/v2/{}",
                txn_ref
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }

    pub async fn repush_deposit_event(&self, txn_ref: String) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!("zainbox/repush/deposit/{}", txn_ref))
            .await?;
        Ok(Response::new(http_response).await)
    }

    pub async fn reconcile_bank_deposit(
        &self,
        verification_type: String,
        bank_type: String,
        account_number: String,
        session_id: Option<String>,
    ) -> Result<Response, Box<dyn Error>> {
        let mut payload = json!({
            "verificationType": verification_type,
            "bankType": bank_type,
            "accountNumber": account_number,
        });

        if let Some(session_id) = session_id {
            payload["sessionId"] = Value::from(session_id);
        }

        let http_response = self
            .engine
            .patch(
                "virtual-account/wallet/transaction/reconcile/bank-deposit",
                &payload,
            )
            .await?;
        Ok(Response::new(http_response).await)
    }
}
