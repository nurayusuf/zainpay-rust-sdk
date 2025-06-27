# Zainpay Rust SDK

[![Crates.io](https://img.shields.io/crates/v/zainpay)](https://crates.io/crates/zainpay)
[![Documentation](https://docs.rs/zainpay/badge.svg)](https://docs.rs/zainpay)
[![License](https://img.shields.io/crates/l/zainpay)](LICENSE)

A Rust implementation of the ZainPay API client.

## Features

- Virtual Account management and transaction
- Zainbox operations and transaction
- Bank listing and operations
- Card payments
- Settlement payments

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zainpay = "0.1"
```

## Usage

```rust
use zainpay::engine::Engine;
use zainpay::enviroment::Environment;
use zainpay::models::model::ZainboxInfo;
use zainpay::zainbox::ZainboxService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let merchant_key ="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9";

    // Initialize the Zainpay client with your API key and environment
    let engine = Engine::new(Environment::Sandbox, merchant_key);

    // Create an instance of the ZainboxService
    let zainbox_service = ZainboxService::new(engine);

    // List Zainboxes with optional filters
    let response = zainbox_service.list(Some(false)).await?;

    // Print the response
    // println!("Zainboxes: {:?}", response);

    // Check if the response is successful
    if response.has_succeeded() {
        // println!("✅ Status Code: {}", response.get_status_code());
        println!("✅ Status: {:?}", response.get_status());
        println!("✅ Code: {:?}", response.get_code());
        println!("✅ Description: {:?}", response.get_description());
        println!("✅ Data: {:?}", response.get_raw_data());

        // convert the data to desired struct
        if let Some(zainboxes) = response.parse_data::<Vec<ZainboxInfo>>() {
            println!("✅ Zainboxes: {:?}", zainboxes);
        } else {
            println!("❌ Could not parse zainboxes data");
        }
    } 
    
    // Check if the response is failure
    else
    if response.has_failed() {
        println!("✅ Status Code: {}", response.get_status_code());
        println!("✅ Status: {:?}", response.get_status());
        println!("✅ Code: {:?}", response.get_code());
        println!("✅ Description: {:?}", response.get_description());
    }
    Ok(())
}
```

## License

MIT