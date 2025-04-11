// Copyright 2019-2025 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_new_transaction_display() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    test_utils::process_test_delay(&config, "NewTransactionDisplay");

    // request object
    let mut request = blockchyp::TransactionDisplayRequest{
        test: true,
        terminal_name: config.default_terminal_name.clone().unwrap_or_else(|| "Test Terminal".to_string()).to_string(),
        transaction: Some(blockchyp::TransactionDisplayTransaction{
            subtotal: "35.00".to_string(),
            tax: "5.00".to_string(),
            total: "70.00".to_string(),
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
    println!("Request: {:?}", request);

    let (response, err) = client.new_transaction_display(&mut request);

    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
}
