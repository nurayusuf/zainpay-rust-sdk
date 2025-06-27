use serde_urlencoded;
use std::collections::HashMap;
pub struct FilterUtil;

impl FilterUtil {

    pub fn construct_filter_params(
        date_from: Option<String>,
        date_to: Option<String>,
        email: Option<String>,
        status: Option<String>,
        txn_ref: Option<String>,
        txn_type: Option<String>,
        payment_channel: Option<String>,
        account_number: Option<String>,
    ) -> String {
        let mut params: HashMap<&'static str, String> = HashMap::new();

        if let Some(date_from) = date_from {
            params.insert("dateFrom", date_from);
        }
        if let Some(date_to) = date_to {
            params.insert("dateTo", date_to);
        }
        if let Some(email) = email {
            params.insert("email", email);
        }
        if let Some(status) = status {
            params.insert("status", status);
        }
        if let Some(txn_ref) = txn_ref {
            params.insert("txnRef", txn_ref);
        }

        if let Some(txn_type) = txn_type {
            params.insert("txnType", txn_type);
        }
        if let Some(payment_channel) = payment_channel {
            params.insert("paymentChannel", payment_channel);
        }

        if let Some(account_number) = account_number {
            params.insert("accountNumber", account_number);
        }

        // Convert HashMap to query string
        serde_urlencoded::to_string(params).unwrap_or_default()
    }
}
