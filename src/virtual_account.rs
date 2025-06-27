use crate::engine::Engine;
use crate::utils::filter::FilterUtil;
use crate::response::Response;
use serde_json::{json};
use std::{error::Error};

pub struct CardService {
    engine: Engine,
}

impl CardService {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    /**
     * Create a virtual account. Map a virtual account to a zainbox. A zainbox can hold multiple virtual accounts.
     *
     * @param string $bankType
     * @param string $bvn
     * @param string $firstName
     * @param string $surname
     * @param string $email
     * @param string $mobile
     * @param string $dob
     * @param string $gender
     * @param string $address
     * @param string $title
     * @param string $state
     * @param string $zainboxCode
     * @return Response
     * @throws GuzzleException
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=create-virtual-account
     */
    pub async fn create(
        &self,
        bank_type: String,
        bvn: String,
        first_name: String,
        last_name: String,
        email: String,
        mobile: String,
        dob: String,
        gender: String,
        address: String,
        title: String,
        state: String,
        zainbox_code: String,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .post(
                "virtual-account/create/request",
                &json!({
                    "bankType": bank_type,
                    "firstName": first_name,
                    "bvn": bvn,
                    "lastName": last_name,
                    "email": email,
                    "mobile": mobile,
                    "dob": dob,
                    "gender":gender,
                    "address": address,
                    "title": title,
                    "state": state,
                    "zainboxCode": zainbox_code
                }),
            )
            .await?;
        Ok(Response::new(http_response).await)
    }

    /**
     * Get the current wallet balance of a virtual account number
     *
     * @param string $accountNumber
     * @return Response
     * @throws GuzzleException
     * @link https://zainpay.ng/developers/api-endpoints?section=virtual-account-balance
     */
    pub async fn get_viirtual_account_balance(&self, account_number: String) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "virtual-account/wallet/balance/{}",
                account_number
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }

    pub async fn get_all_virtual_accounts_balance_for_zainbox(
        &self,
        zainbox_code: String,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!("zainbox/accounts/balance/{}", zainbox_code))
            .await?;
        Ok(Response::new(http_response).await)
    }

    pub async fn change_virtual_account_status(
        &self,
        zainbox_code: String,
        account_number: String,
        status: bool,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .patch(
                "/virtual-account/change/account/status",
                &json!({
                    "zainboxCode": zainbox_code,
                    "accountNumber": account_number,
                    "status": status
                }),
            )
            .await?;
        Ok(Response::new(http_response).await)
    }

     /**
     * This endpoint fetches all current account balances for all virtual accounts in a zainbox.
     *
     * @param string $zainboxCode
     * @return Response
     * @throws GuzzleException
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=zainbox-virtual-accounts-balances
     */

     pub async fn get_all_virtual_accounts_for_zainbox(
        &self,
        zainbox_code: String,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!("zainbox/virtual-accounts/{}", zainbox_code))
            .await?;
        Ok(Response::new(http_response).await)
    }

    /**
     * Get all transactions of an account
     *
     * @param string $accountNumber
     * @return Response
     * @throws GuzzleException
     * @link https://zainpay.ng/developers/api-endpoints?section=virtual-account-transactions
     */
    pub async fn get_virtual_account_txn_history(
        &self,
        account_number: String,
        count: Option<u32>,
        date_from: Option<String>,
        date_to: Option<String>,
        txn_type: Option<String>,
        payment_channel: Option<String>,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "virtual-account/wallet/transactions/{}/{}?{}",
                account_number,
                count.unwrap_or(20),
                FilterUtil::construct_filter_params(
                    date_from,
                    date_to,
                    None,
                    None,
                    None,
                    txn_type,
                    payment_channel,
                    None
                )
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }
}
