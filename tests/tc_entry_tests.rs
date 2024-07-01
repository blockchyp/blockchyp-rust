// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_tc_entry() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    // setup request object
    let setup_request = blockchyp::TermsAndConditionsLogRequest{

        ..Default::default()
    };
    println!("Setup Request: {:?}", setup_request);

    let (setup_response, err) = client.tc_log(&setup_request);

    assert!(err.is_none());

    println!("Setup Response: {:?}", setup_response);

    // request object
    let request = blockchyp::TermsAndConditionsLogRequest{
        log_entry_id: setup_response.results.as_ref().unwrap_or_else(|| panic!("Expected Some(vec) but got None"))[0].id.to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.tc_entry(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
    assert!(!response.id.is_empty());
    assert!(!response.terminal_id.is_empty());
    assert!(!response.terminal_name.is_empty());
    assert!(!response.timestamp.is_empty());
    assert!(!response.name.is_empty());
    assert!(!response.content.is_empty());
    assert!(response.has_signature);
    assert!(!response.signature.is_empty());
}
