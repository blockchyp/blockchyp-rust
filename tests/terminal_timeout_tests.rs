// Copyright 2019-2025 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_terminal_timeout() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    test_utils::process_test_delay(&config, "TerminalTimeout");

    // request object
    let mut request = blockchyp::AuthorizationRequest{
        timeout: 1,
        terminal_name: config.default_terminal_name.clone().unwrap_or_else(|| "Test Terminal".to_string()).to_string(),
        amount: "25.15".to_string(),
        test: true,
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.charge(&mut request);
    println!("Response: {:?}", response);
    println!("Response Error: {:?}", err);

    assert!(err.is_some());

    assert_eq!(response.response_description, blockchyp::RESPONSE_TIMED_OUT);


}
