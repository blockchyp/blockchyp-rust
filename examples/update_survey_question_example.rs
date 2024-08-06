use blockchyp;
use std::error::Error;

fn update_survey_question_example() -> Result<(), Box<dyn Error>> {
    // sample credentials
    let creds = blockchyp::APICredentials {
        api_key: "ZDSMMZLGRPBPRTJUBTAFBYZ33Q".to_string(),
        bearer_token: "ZLBW5NR4U5PKD5PNP3ZP3OZS5U".to_string(),
        signing_key: "9c6a5e8e763df1c9256e3d72bd7f53dfbd07312938131c75b3bfd254da787947".to_string(),
    };

    // instantiate the client
    let client = blockchyp::Client::new(creds);

    let request = blockchyp::SurveyQuestion{
        id: "<QUESTION ID>".to_string(),
        ordinal: 1,
        question_text: "Would you shop here again?".to_string(),
        question_type: "yes_no".to_string(),
        enabled: true,
        ..Default::default()
    };
    let (response, err) = client.update_survey_question(&request);

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
    update_survey_question_example()?;
    println!("Example completed successfully!");
    Ok(())
}
