// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use ::blockchyp::*;

#[test]
fn test_pan_preauth() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    test_utils::process_test_delay(&config, "PANPreauth");

    // request object
    let mut request = AuthorizationRequest{
        pan: "4111111111111111".to_string(),
        exp_month: "12".to_string(),
        exp_year: "2025".to_string(),
        amount: "42.45".to_string(),
        test: true,
        bypass_dupe_filter: true,
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.preauth(&mut request);

    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
    assert!(response.approved);
    assert!(response.test);
    assert_eq!(response.auth_code.len(), 6);
    assert!(!response.transaction_id.is_empty());
    assert!(!response.timestamp.is_empty());
    assert!(!response.tick_block.is_empty());
    assert_eq!(response.response_description, "approved");
    assert!(!response.payment_type.is_empty());
    assert!(!response.masked_pan.is_empty());
    assert!(!response.entry_method.is_empty());
    assert_eq!(response.entry_method, "KEYED");
}
