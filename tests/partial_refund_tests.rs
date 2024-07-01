// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_partial_refund() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    test_utils::process_test_delay(&config, "PartialRefund");

    // setup request object
    let mut setup_request = blockchyp::AuthorizationRequest{
        pan: "4111111111111111".to_string(),
        exp_month: "12".to_string(),
        exp_year: "2025".to_string(),
        amount: "25.55".to_string(),
        test: true,
        transaction_ref: test_utils::random_id().to_string(),
        ..Default::default()
    };
    println!("Setup Request: {:?}", setup_request);

    let (setup_response, err) = client.charge(&mut setup_request);

    assert!(err.is_none());

    println!("Setup Response: {:?}", setup_response);

    // request object
    let mut request = blockchyp::RefundRequest{
        transaction_id: setup_response.transaction_id.to_string(),
        amount: "5.00".to_string(),
        test: true,
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.refund(&mut request);

    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
    assert!(response.approved);
}
