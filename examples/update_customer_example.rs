use blockchyp;
use std::error::Error;

fn update_customer_example() -> Result<(), Box<dyn Error>> {
    // sample credentials
    let creds = blockchyp::APICredentials {
        api_key: "ZDSMMZLGRPBPRTJUBTAFBYZ33Q".to_string(),
        bearer_token: "ZLBW5NR4U5PKD5PNP3ZP3OZS5U".to_string(),
        signing_key: "9c6a5e8e763df1c9256e3d72bd7f53dfbd07312938131c75b3bfd254da787947".to_string(),
    };

    // instantiate the client
    let client = blockchyp::Client::new(creds);

    let request = blockchyp::UpdateCustomerRequest{
        customer: blockchyp::Customer{
            id: "<CUSTOMER ID>".to_string(),
            customer_ref: "Customer reference string".to_string(),
            first_name: "FirstName".to_string(),
            last_name: "LastName".to_string(),
            company_name: "Company Name".to_string(),
            email_address: "notifications@blockchypteam.m8r.co".to_string(),
            sms_number: "(123) 123-1231".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    let (response, err) = client.update_customer(&request);

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
    update_customer_example()?;
    println!("Example completed successfully!");
    Ok(())
}
