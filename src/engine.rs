use crate::enviroment::Environment;
use reqwest::{Client, Response as ReqwestResponse};
use std::error::Error;

#[derive(Clone)]
pub struct Engine {
    client: Client,
    pub base_url: String,
    pub merchant_key: String,
}

impl Engine {
    pub fn new(enviroment: Environment, merchant_key: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: enviroment.base_url().to_string(),
            merchant_key: merchant_key.to_string(),
        }
    }

    pub async fn post<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<ReqwestResponse, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, path);
        Ok(self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.merchant_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?)
    }

    pub async fn get(&self, path: &str) -> Result<ReqwestResponse, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, path);
        Ok(self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.merchant_key))
            .send()
            .await?)
    }

    pub async fn patch<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<ReqwestResponse, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, path);
        Ok(self
            .client
            .patch(url)
            .header("Authorization", format!("Bearer {}", self.merchant_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?)
    }
}
