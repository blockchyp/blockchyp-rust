// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use ::blockchyp::*;

#[test]
fn test_tc_delete_template() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    // setup request object
    let setup_request = TermsAndConditionsTemplate{
        alias: test_utils::random_id().to_string(),
        name: "HIPPA Disclosure".to_string(),
        content: "Lorem ipsum dolor sit amet.".to_string(),
        ..Default::default()
    };
    println!("Setup Request: {:?}", setup_request);

    let (setup_response, err) = client.tc_update_template(&setup_request);

    assert!(err.is_none());

    println!("Setup Response: {:?}", setup_response);

    // request object
    let request = TermsAndConditionsTemplateRequest{
        template_id: setup_response.id.to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.tc_delete_template(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
}
