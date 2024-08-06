use blockchyp;
use std::error::Error;

fn terms_and_conditions_example() -> Result<(), Box<dyn Error>> {
    // sample credentials
    let creds = blockchyp::APICredentials {
        api_key: "ZDSMMZLGRPBPRTJUBTAFBYZ33Q".to_string(),
        bearer_token: "ZLBW5NR4U5PKD5PNP3ZP3OZS5U".to_string(),
        signing_key: "9c6a5e8e763df1c9256e3d72bd7f53dfbd07312938131c75b3bfd254da787947".to_string(),
    };

    // instantiate the client
    let client = blockchyp::Client::new(creds);

    let mut request = blockchyp::TermsAndConditionsRequest{
        test: true,
        terminal_name: "Test Terminal".to_string(),
        tc_alias: "hippa".to_string(),
        tc_name: "HIPPA Disclosure".to_string(),
        tc_content: "Full contract text".to_string(),
        sig_format: blockchyp::SignatureFormat::PNG,
        sig_width: 200,
        sig_required: true,
        ..Default::default()
    };
    let (response, err) = client.terms_and_conditions(&mut request);

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
    terms_and_conditions_example()?;
    println!("Example completed successfully!");
    Ok(())
}
