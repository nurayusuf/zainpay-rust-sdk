use serde_json::Value;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Response {
    status_code: u16,
    decoded_response: Option<HashMap<String, Value>>,
    error: bool,
    error_message: Option<String>,
}

impl Response {
    pub async fn new(response: reqwest::Response) -> Self {
        let status_code = response.status().as_u16();
        let body_result = response.text().await;
        println!("Response body: {:?}", body_result);
        
        let decoded_response = body_result
            .ok()
            .and_then(|body| serde_json::from_str::<HashMap<String, Value>>(&body).ok());
        
        let error = status_code >= 400;
        
        Self {
            status_code,
            decoded_response,
            error,
            error_message: None,
        }
    }

    pub fn has_succeeded(&self) -> bool {
        !self.error && matches!(self.get_code().as_deref(), Some("200") | Some("00") | Some("21"))
    }

    pub fn has_failed(&self) -> bool {
        self.error || !self.has_succeeded()
    }

    pub fn get_status(&self) -> Option<&str> {
        self.decoded_response
            .as_ref()
            .and_then(|map| map.get("status")?.as_str())
    }

    pub fn get_code(&self) -> Option<&str> {
        self.decoded_response
            .as_ref()
            .and_then(|map| map.get("code")?.as_str())
    }

    pub fn get_description(&self) -> Option<&str> {
        self.decoded_response
            .as_ref()
            .and_then(|map| map.get("description")?.as_str())
    }

    pub fn get_raw_data(&self) -> Option<&Value> {
        self.decoded_response
            .as_ref()
            .and_then(|map| map.get("data"))
    }

    pub fn parse_data<T: DeserializeOwned>(&self) -> Option<T> {
        self.get_raw_data()
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    pub fn get_status_code(&self) -> u16 {
        self.status_code
    }

    pub fn set_error(&mut self, error: bool) {
        self.error = error;
    }

    pub fn set_error_message(&mut self, error_message: String) {
        self.error_message = Some(error_message);
    }

    pub fn has_error(&self) -> bool {
        self.error
    }

    pub fn get_error_message(&self) -> Option<&String> {
        self.error_message.as_ref()
    }

    pub fn full_json(&self) -> Option<&HashMap<String, Value>> {
        self.decoded_response.as_ref()
    }
}