// Copyright 2019-2026 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_add_test_merchant() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some("partner"));

    // request object
    let request = blockchyp::AddTestMerchantRequest{
        dba_name: "Test Merchant".to_string(),
        company_name: "Test Merchant".to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.add_test_merchant(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
    assert_eq!(response.dba_name, "Test Merchant");
    assert_eq!(response.company_name, "Test Merchant");
    assert!(response.visa);
}
