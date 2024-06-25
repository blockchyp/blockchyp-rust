use ::blockchyp::*;
use std::error::Error;

fn update_merchant_example() -> Result<(), Box<dyn Error>> {
    // sample credentials
    let creds = APICredentials {
        api_key: "ZDSMMZLGRPBPRTJUBTAFBYZ33Q".to_string(),
        bearer_token: "ZLBW5NR4U5PKD5PNP3ZP3OZS5U".to_string(),
        signing_key: "9c6a5e8e763df1c9256e3d72bd7f53dfbd07312938131c75b3bfd254da787947".to_string(),
    };

    // instantiate the client
    let client = Client::new(creds);

    let request = MerchantProfile{
        merchant_id: "<MERCHANT ID>".to_string(),
        test: true,
        dba_name: "Test Merchant".to_string(),
        company_name: "Test Merchant".to_string(),
        billing_address: Address{
            address_1: "1060 West Addison".to_string(),
            city: "Chicago".to_string(),
            state_or_province: "IL".to_string(),
            postal_code: "60613".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    let (response, err) = client.update_merchant(&request);

    if let Some(e) = err {
        eprintln!("Unexpected error occurred: {:?}", e);
        return Err(e)
    }

    if response.success {
		println!("Success");
	}

    println!("Response: {:?}", response);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    update_merchant_example()?;
    println!("Example completed successfully!");
    Ok(())
}
