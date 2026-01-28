// Copyright 2019-2026 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;
use std::fs::File;
#[test]
fn test_update_slide_show() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    // setup request object
    let setup_request = blockchyp::UploadMetadata{
        file_name: "aviato.png".to_string(),
        file_size: 18843,
        upload_id: test_utils::random_id().to_string(),
        ..Default::default()
    };
    println!("Setup Request: {:?}", setup_request);

    let mut file = match File::open("tests/testdata/aviato.png") {
        Ok(file) => file,
        Err(e) => {
            panic!("Unable to open file: {:?}", e);
        }
    };
    let (setup_response, err) = client.upload_media(&setup_request, &mut file);

    assert!(err.is_none());

    println!("Setup Response: {:?}", setup_response);

    // request object
    let request = blockchyp::SlideShow{
        name: "Test Slide Show".to_string(),
        delay: 5,
        slides: Some(vec![
            blockchyp::Slide{
                media_id: setup_response.id.to_string(),
                ..Default::default()
            },

        ]),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.update_slide_show(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
    assert_eq!(response.name, "Test Slide Show");
}
