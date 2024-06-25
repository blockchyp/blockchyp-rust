// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use ::blockchyp::*;
use std::fs::File;
#[test]
fn test_media_upload() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    // request object
    let request = UploadMetadata{
        file_name: "aviato.png".to_string(),
        file_size: 18843,
        upload_id: test_utils::random_id().to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let mut file = match File::open("tests/testdata/aviato.png") {
        Ok(file) => file,
        Err(e) => {
            panic!("Unable to open file: {:?}", e);
        }
    };
    let (response, err) = client.upload_media(&request, &mut file);

    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
    assert!(!response.id.is_empty());
    assert_eq!(response.original_file, "aviato.png");
    assert!(!response.file_url.is_empty());
    assert!(!response.thumbnail_url.is_empty());
}
