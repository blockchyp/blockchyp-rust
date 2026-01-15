// Copyright 2019-2026 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_gateway_timeout() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    test_utils::process_test_delay(&config, "GatewayTimeout");

    // request object
    let mut request = blockchyp::AuthorizationRequest{
        timeout: 1,
        pan: "5555555555554444".to_string(),
        exp_month: "12".to_string(),
        exp_year: "2025".to_string(),
        amount: "25.55".to_string(),
        test: true,
        transaction_ref: test_utils::random_id().to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.charge(&mut request);
    println!("Response: {:?}", response);
    println!("Response Error: {:?}", err);

    assert!(err.is_some());

    assert_eq!(response.response_description, blockchyp::RESPONSE_TIMED_OUT);


}
