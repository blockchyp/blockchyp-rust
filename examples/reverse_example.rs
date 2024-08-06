use blockchyp;
use std::error::Error;

fn reverse_example() -> Result<(), Box<dyn Error>> {
    // sample credentials
    let creds = blockchyp::APICredentials {
        api_key: "ZDSMMZLGRPBPRTJUBTAFBYZ33Q".to_string(),
        bearer_token: "ZLBW5NR4U5PKD5PNP3ZP3OZS5U".to_string(),
        signing_key: "9c6a5e8e763df1c9256e3d72bd7f53dfbd07312938131c75b3bfd254da787947".to_string(),
    };

    // instantiate the client
    let client = blockchyp::Client::new(creds);

    let request = blockchyp::AuthorizationRequest{
        transaction_ref: "<LAST TRANSACTION REF>".to_string(),
        ..Default::default()
    };
    let (response, err) = client.reverse(&request);

    if let Some(e) = err {
        eprintln!("Unexpected error occurred: {:?}", e);
        return Err(e)
    }

    if response.approved {
		println!("approved");
	}

    println!("Response: {:?}", response);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    reverse_example()?;
    println!("Example completed successfully!");
    Ok(())
}
