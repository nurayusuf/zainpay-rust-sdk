# Zainpay Rust SDK

[![Crates.io](https://img.shields.io/crates/v/zainpay)](https://crates.io/crates/zainpay)
[![Documentation](https://docs.rs/zainpay/badge.svg)](https://docs.rs/zainpay)
[![License](https://img.shields.io/crates/l/zainpay)](LICENSE)

A Rust implementation of the ZainPay API client.

## Features

- Virtual Account management
- Zainbox operations
- Bank listing and account resolution
- Card payments
- Engine status checks

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zainpay = "0.1"
```

## Usage

```rust
use zainpay::{ZainpayClient, virtual_account::CreateVirtualAccountRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ZainpayClient::new("your_api_key".to_string(), None)?;
    
    // Create virtual account
    let request = CreateVirtualAccountRequest {
        wallet_id: "wallet123".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        phone_number: "08012345678".to_string(),
        dob: "1990-01-01".to_string(),
        gender: "Male".to_string(),
        address: "123 Main St".to_string(),
    };
    
    let account = client.create_virtual_account(request).await?;
    println!("Created virtual account: {:?}", account);
    
    Ok(())
}
```

## License

MIT