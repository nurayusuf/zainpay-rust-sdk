use crate::engine::Engine;
use crate::response::Response;
use crate::utils::filter::FilterUtil;
use serde_json::json;
use std::error::Error;

pub struct SettlementService {
    engine: Engine,
}
use crate::models::model::SettlementAccount;

impl SettlementService {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    /**
     * For Scheduling Settlement
     *
     * Create a scheduled settlement for a zainbox
     * To create a scheduled settlement for a zainbox., please bear in mind that at any given time, a zainbox can only have one type of settlement.
     * Planned settlements are divided into three categories.
     *
     * Check the docs out for more descriptive information.
     *
     * @param string $name
     * @param string $zainboxCode
     * @param string $scheduleType
     * @param string $schedulePeriod
     * @param array $settlementAccountList
     * @param bool $status
     * @return Response
     * @throws GuzzleException
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=create-settlement
     */
    pub async fn create_or_update_zainbox_settlemet(
        &self,
        name: String,
        zainbox_code: String,
        schedule_type: String,
        schedule_period: String,
        settlement_account_list: Vec<SettlementAccount>,
        status: bool,
    ) -> Result<Response, Box<dyn Error>> {
        let payload = json!({
            "name": name,
            "zainboxCode": zainbox_code,
            "scheduleType": schedule_type,
            "schedulePeriod": schedule_period,
            "settlementAccountList": settlement_account_list,
            "status": status,
        });

        let http_response = self.engine.post("zainbox/settlement", &payload).await?;
        Ok(Response::new(http_response).await)
    }

    /**
     * For getting settlement(s) tied to a zainbox
     *
     * @param string $zainboxCode
     * @return Response
     * @throws GuzzleException
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=get-settlement
     */
    pub async fn get_settlement_info_for_zainbox(
        &self,
        zainbox_code: String,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!("zainbox/settlement?zainboxCode={}", zainbox_code))
            .await?;
        Ok(Response::new(http_response).await)
    }

    /**
     * Get the sum of total amount collected by all virtual accounts for a merchant in a particular period,
     * for both transfer and deposit transactions
     *
     * @param string $zainboxCode
     * @param int $count
     * @param string $status
     * @param string $dateFrom
     * @param string $dateTo
     * @return Response
     * @throws GuzzleException
     *
     * @link https://zainpay.ng/developers/api-endpoints?section=settment-payments-by-zainbox
     */
    pub async fn get_settlement_payment_history_for_zainbox(
        &self,
        zainbox_code: String,
        count: Option<u32>,
        date_from: Option<String>,
        date_to: Option<String>,
        status: Option<String>,
    ) -> Result<Response, Box<dyn Error>> {
        let http_response = self
            .engine
            .get(&format!(
                "zainbox/settlement/history/{}?{}&{}",
                zainbox_code,
                count.unwrap_or(20),
                FilterUtil::construct_filter_params(
                    date_from, date_to, None, status, None, None, None, None
                )
            ))
            .await?;
        Ok(Response::new(http_response).await)
    }

    pub fn settlement_account_payload(
        account_number: String,
        bank_code: String,
        percentage: f64,
    ) -> SettlementAccount {
        SettlementAccount::new(account_number, bank_code, percentage)
    }
}
