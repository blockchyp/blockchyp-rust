// Copyright 2019-2026 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;
use std::fs::File;
#[test]
fn test_update_branding_asset() {
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
    let request = blockchyp::BrandingAsset{
        media_id: setup_response.id.to_string(),
        padded: true,
        ordinal: 10,
        start_date: "01/06/2021".to_string(),
        start_time: "14:00".to_string(),
        end_date: "11/05/2024".to_string(),
        end_time: "16:00".to_string(),
        notes: "Test Branding Asset".to_string(),
        preview: false,
        enabled: true,
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.update_branding_asset(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
}
