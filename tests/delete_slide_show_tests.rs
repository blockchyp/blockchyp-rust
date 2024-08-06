// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_delete_slide_show() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    // setup request object
    let setup_request = blockchyp::SlideShow{
        name: "Test Slide Show".to_string(),
        delay: 5,
        ..Default::default()
    };
    println!("Setup Request: {:?}", setup_request);

    let (setup_response, err) = client.update_slide_show(&setup_request);

    assert!(err.is_none());

    println!("Setup Response: {:?}", setup_response);

    // request object
    let request = blockchyp::SlideShowRequest{
        slide_show_id: setup_response.id.to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.delete_slide_show(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
}
