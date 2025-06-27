use crate::engine::Engine;
use crate::utils::filter::FilterUtil;
use crate::response::Response;
use serde_json::{json, Value};
use std::error::Error;

pub struct ZainboxService {
    engine: Engine,
}

impl ZainboxService {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    /**
     * Create a Zainbox
     *
     * @param string $name
     * @param string $emailNotification
     * @param array|null $tags
     * @param string $callbackUrl
     * @param string|null $description
     * @param string|null $codeNamePrefix
     * @param bool|null $allowAutoInternalTransfer
     * @return Response
     * @throws Box<dyn Error>
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=create-zainbox
     */
    pub async fn create(
        &self,
        name: String,
        email_notification: String,
        tags: Option<Vec<String>>,
        callback_url: String,
        description: Option<String>,
        code_name_prefix: Option<String>,
        allow_auto_internal_transfer: Option<bool>,
    ) -> Result<Response, Box<dyn Error>> {
        let mut payload = json!({
            "name": name,
            "emailNotification": email_notification,
            "callbackUrl": callback_url,
        });

        if let Some(tags) = tags {
            payload["tags"] = Value::from(tags.join(","));
        }
        if let Some(code_name_prefix) = code_name_prefix {
            payload["codeNamePrefix"] = Value::from(code_name_prefix);
        }
        if let Some(description) = description {
            payload["description"] = Value::from(description);
        }
        if let Some(allow_auto_internal_transfer) = allow_auto_internal_transfer {
            payload["allowAutoInternalTransfer"] = Value::from(allow_auto_internal_transfer);
        }

        let http_response = self.engine.post("zainbox/create/request", &payload).await?;
        Ok(Response::new(http_response).await)
    }

    pub async fn list(&self, status: Option<bool>) -> Result<Response, Box<dyn Error>> {
        let http_response = match status {
            Some(s) => self.engine.get(&format!("zainbox/list?status={}", s)).await?,
            None => self.engine.get("zainbox/list").await?,
        };
        Ok(Response::new(http_response).await)
    }

    /**
     * Update a Zainbox
     *
     * @param string $name
     * @param string|null $emailNotification
     * @param array|null $tags
     * @param string|null $callbackUrl
     * @param string|null $description
     * @param bool|null $allowAutoInternalTransfer
     * @param string $zainboxCode
     * @param bool|null $status
     * @return Response
     * @throws Box<dyn Error>
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=update-zainbox
     */
    pub async fn update(
        &self,
        name: String,
        email_notification: Option<String>,
        tags: Option<Vec<String>>,
        callback_url: Option<String>,
        description: Option<String>,
        allow_auto_internal_transfer: Option<bool>,
        zainbox_code: String,
        status: Option<bool>,
    ) -> Result<Response, Box<dyn Error>> {
        let mut payload = json!({
            "codeName": zainbox_code,
            "name": name,
        });

        if let Some(tags) = tags {
            payload["tags"] = Value::from(tags.join(","));
        }
        if let Some(callback_url) = callback_url {
            payload["callbackUrl"] = Value::from(callback_url);
        }
        if let Some(email_notification) = email_notification {
            payload["emailNotification"] = Value::from(email_notification);
        }
        if let Some(description) = description {
            payload["description"] = Value::from(description);
        }
        if let Some(allow_auto_internal_transfer) = allow_auto_internal_transfer {
            payload["allowAutoInternalTransfer"] = Value::from(allow_auto_internal_transfer);
        }
        if let Some(status) = status {
            payload["status"] = Value::from(status);
        }

        let http_response = self.engine.patch("zainbox/update", &payload).await?;
        Ok(Response::new(http_response).await)
    }

    /**
     *  Get the complete profile of a Zainbox, including the Current Billing Plan for account to account and interBank transfers respectively
     *
     * @param string $zainboxCode
     * @return Response
     * @throws Box<dyn Error>
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=zainbox-profile
     */
    pub async fn get_zainbox_profile(
        &self,
        zainbox_code: String,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!("zainbox/profile/{}", zainbox_code))
            .await?;
        Ok(Response::new(http_response).await)
    }

    /**
     * Get the sum of total amount collected by all virtual accounts for a particular zainbox in a particular period,
     * for both transfer and deposit transactions
     *
     * @param string $zainboxCode
     * @param string $dateFrom
     * @param string $dateTo
     * @return Response
     * @throws Box<dyn Error>
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=total-payment-by-zainbox
     */
    pub async fn get_total_payment_collected_by_zainbox(
        &self,
        zainbox_code: String,
        date_from: Option<String>,
        date_to: Option<String>,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "zainbox/transfer/deposit/summary/{}?{}",
                zainbox_code,
                FilterUtil::construct_filter_params(
                    date_from, date_to, None, None, None, None, None, None
                )
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }

    /**
     * Get the sum of total amount collected by all virtual accounts for a merchant in a particular period,
     * for both transfer and deposit transactions
     *
     * @param string $dateFrom
     * @param string $dateTo
     * @return Response
     * @throws Box<dyn Error>
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=total-payment-by-merchant
     */
    pub async fn get_total_payment_collected_for_all_zainboxes(
        &self,
        date_from: Option<String>,
        date_to: Option<String>,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "zainbox/transactions/summary?{}",
                FilterUtil::construct_filter_params(
                    date_from, date_to, None, None, None, None, None, None
                )
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }

    /**
     * Get a list of transactions from a particular zainbox
     *
     * @param string $zainboxCode
     * @return Response
     * @throws Box<dyn Error>
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=zainbox-transactions-history
     */
    pub async fn get_zainbox_txn_history(
        &self,
        zainbox_code: String,
        count: Option<u32>,
        date_from: Option<String>,
        date_to: Option<String>,
        txn_type: Option<String>,
        payment_channel: Option<String>,
        account_number: Option<String>,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "zainbox/transactions/{}/{}?{}",
                zainbox_code,
                count.unwrap_or(20),
                FilterUtil::construct_filter_params(
                    date_from,
                    date_to,
                    None,
                    None,
                    None,
                    txn_type,
                    payment_channel,
                    account_number
                )
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }

    /**
     *  Get the list of first 20 transactions from all zainboxes in descending order of date
     *
     * @return Response
     * @throws Box<dyn Error>
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=merchant-transactions
     */
    pub async fn get_all_zainboxes_txn_history(
        &self,
        count: Option<u32>,
        date_from: Option<String>,
        date_to: Option<String>,
        txn_type: Option<String>,
        payment_channel: Option<String>,
        account_number: Option<String>,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "zainbox/transactions?count={}&{}",
                count.unwrap_or(20),
                FilterUtil::construct_filter_params(
                    date_from,
                    date_to,
                    None,
                    None,
                    None,
                    txn_type,
                    payment_channel,
                    account_number
                )
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }
}
