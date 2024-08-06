// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_update_merchant() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some("partner"));

    // request object
    let request = blockchyp::MerchantProfile{
        test: true,
        dba_name: "Test Merchant".to_string(),
        company_name: "Test Merchant".to_string(),
        billing_address: blockchyp::Address{
            address_1: "1060 West Addison".to_string(),
            city: "Chicago".to_string(),
            state_or_province: "IL".to_string(),
            postal_code: "60613".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.update_merchant(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
}
