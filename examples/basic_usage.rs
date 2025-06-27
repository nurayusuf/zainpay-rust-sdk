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

    // let response = zainbox_service.get_zainbox_txn_history("I0thsos35C15mZM6lusm".to_string(), None, None, None,None, None, None).await?;



    // Print the response
    // println!("Zainboxes: {:?}", response);

    // Check if the response is successful
    if response.has_succeeded() {
        // println!("✅ Status Code: {}", response.get_status_code());
        println!("✅ Status: {:?}", response.get_status());
        println!("✅ Code: {:?}", response.get_code());
        println!("✅ Description: {:?}", response.get_description());
        println!("✅ Data: {:?}", response.get_raw_data());

        if let Some(zainboxes) = response.parse_data::<Vec<ZainboxInfo>>() {
            println!("✅ Zainboxes: {:?}", zainboxes);
        } else {
            println!("❌ Could not parse payment data");
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
