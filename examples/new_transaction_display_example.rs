use blockchyp;
use std::error::Error;

fn new_transaction_display_example() -> Result<(), Box<dyn Error>> {
    // sample credentials
    let creds = blockchyp::APICredentials {
        api_key: "ZDSMMZLGRPBPRTJUBTAFBYZ33Q".to_string(),
        bearer_token: "ZLBW5NR4U5PKD5PNP3ZP3OZS5U".to_string(),
        signing_key: "9c6a5e8e763df1c9256e3d72bd7f53dfbd07312938131c75b3bfd254da787947".to_string(),
    };

    // instantiate the client
    let client = blockchyp::Client::new(creds);

    let mut request = blockchyp::TransactionDisplayRequest{
        test: true,
        terminal_name: "Test Terminal".to_string(),
        transaction: Some(blockchyp::TransactionDisplayTransaction{
            subtotal: "60.00".to_string(),
            tax: "5.00".to_string(),
            total: "65.00".to_string(),
            items: Some(vec![
                blockchyp::TransactionDisplayItem{
                    description: "Leki Trekking Poles".to_string(),
                    price: "35.00".to_string(),
                    quantity: 2.0,
                    extended: "70.00".to_string(),
                    discounts: Some(vec![
                        blockchyp::TransactionDisplayDiscount{
                            description: "memberDiscount".to_string(),
                            amount: "10.00".to_string(),
                            ..Default::default()
                        },

                    ]),
                    ..Default::default()
                },

            ]),
            ..Default::default()
        }),
        ..Default::default()
    };
    let (response, err) = client.new_transaction_display(&mut request);

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
    new_transaction_display_example()?;
    println!("Example completed successfully!");
    Ok(())
}
