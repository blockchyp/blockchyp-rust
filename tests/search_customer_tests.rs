// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use ::blockchyp::*;

#[test]
fn test_search_customer() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    // setup request object
    let setup_request = UpdateCustomerRequest{
        customer: Customer{
            first_name: "Test".to_string(),
            last_name: "Customer".to_string(),
            company_name: "Test Company".to_string(),
            email_address: "support@blockchyp.com".to_string(),
            sms_number: "(123) 123-1234".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    println!("Setup Request: {:?}", setup_request);

    let (setup_response, err) = client.update_customer(&setup_request);

    assert!(err.is_none());

    println!("Setup Response: {:?}", setup_response);

    // request object
    let request = CustomerSearchRequest{
        query: "123123".to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.customer_search(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
}
