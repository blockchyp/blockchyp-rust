// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use ::blockchyp::*;

#[test]
fn test_update_merchant_platforms() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some("partner"));

    // setup request object
    let setup_request = AddTestMerchantRequest{
        dba_name: "Test Merchant".to_string(),
        company_name: "Test Merchant".to_string(),
        ..Default::default()
    };
    println!("Setup Request: {:?}", setup_request);

    let (setup_response, err) = client.add_test_merchant(&setup_request);

    assert!(err.is_none());

    println!("Setup Response: {:?}", setup_response);

    // request object
    let request = MerchantPlatform{
        merchant_id: setup_response.merchant_id.to_string(),
        platform_code: "SIM".to_string(),
        notes: "platform simulator".to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.update_merchant_platforms(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
}
