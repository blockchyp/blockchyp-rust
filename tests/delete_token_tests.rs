// Copyright 2019-2025 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_delete_token() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    // setup request object
    let mut setup_request = blockchyp::EnrollRequest{
        pan: "4111111111111111".to_string(),
        test: true,
        customer: Some(blockchyp::Customer{
            customer_ref: "TESTCUSTOMER".to_string(),
            first_name: "Test".to_string(),
            last_name: "Customer".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };
    println!("Setup Request: {:?}", setup_request);

    let (setup_response, err) = client.enroll(&mut setup_request);

    assert!(err.is_none());

    println!("Setup Response: {:?}", setup_response);

    // request object
    let request = blockchyp::DeleteTokenRequest{
        token: setup_response.token.to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.delete_token(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
}
