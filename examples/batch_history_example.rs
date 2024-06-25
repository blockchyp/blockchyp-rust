use ::blockchyp::*;
use std::error::Error;

fn batch_history_example() -> Result<(), Box<dyn Error>> {
    // sample credentials
    let creds = APICredentials {
        api_key: "ZDSMMZLGRPBPRTJUBTAFBYZ33Q".to_string(),
        bearer_token: "ZLBW5NR4U5PKD5PNP3ZP3OZS5U".to_string(),
        signing_key: "9c6a5e8e763df1c9256e3d72bd7f53dfbd07312938131c75b3bfd254da787947".to_string(),
    };

    // instantiate the client
    let client = Client::new(creds);

    let request = BatchHistoryRequest{
        max_results: 250,
        start_index: 0,
        ..Default::default()
    };
    let (response, err) = client.batch_history(&request);

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
    batch_history_example()?;
    println!("Example completed successfully!");
    Ok(())
}
