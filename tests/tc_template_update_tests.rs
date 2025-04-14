// Copyright 2019-2025 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_tc_template_update() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    // request object
    let request = blockchyp::TermsAndConditionsTemplate{
        alias: test_utils::random_id().to_string(),
        name: "HIPPA Disclosure".to_string(),
        content: "Lorem ipsum dolor sit amet.".to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.tc_update_template(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
    assert!(!response.alias.is_empty());
    assert_eq!(response.name, "HIPPA Disclosure");
    assert_eq!(response.content, "Lorem ipsum dolor sit amet.");
}
