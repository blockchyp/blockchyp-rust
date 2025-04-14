// Copyright 2019-2025 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_deactivate_terminal() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    // request object
    let request = blockchyp::TerminalDeactivationRequest{
        terminal_id: test_utils::random_id().to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.deactivate_terminal(&request);
    assert!(err.is_some());


    println!("Response: {:?}", response);

    // response assertions
    assert!(!response.success);
}
