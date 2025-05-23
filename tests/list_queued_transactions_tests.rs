// Copyright 2019-2025 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_list_queued_transactions() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    test_utils::process_test_delay(&config, "ListQueuedTransactions");

    // setup request object
    let mut setup_request = blockchyp::AuthorizationRequest{
        terminal_name: config.default_terminal_name.clone().unwrap_or_else(|| "Test Terminal".to_string()).to_string(),
        transaction_ref: test_utils::random_id().to_string(),
        description: "1060 West Addison".to_string(),
        amount: "25.15".to_string(),
        test: true,
        queue: true,
        ..Default::default()
    };
    println!("Setup Request: {:?}", setup_request);

    let (setup_response, err) = client.charge(&mut setup_request);

    assert!(err.is_none());

    println!("Setup Response: {:?}", setup_response);

    // request object
    let mut request = blockchyp::ListQueuedTransactionsRequest{
        terminal_name: config.default_terminal_name.clone().unwrap_or_else(|| "Test Terminal".to_string()).to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.list_queued_transactions(&mut request);

    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
}
