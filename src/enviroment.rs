#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Environment {
    Localbox,
    Sandbox,
    Production,
}

impl Environment {
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Sandbox => "https://sandbox.zainpay.ng",
            Environment::Production => "https://api.zainpay.ng",
        }
    }
}