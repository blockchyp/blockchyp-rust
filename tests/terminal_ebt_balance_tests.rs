// Copyright 2019-2025 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_terminal_ebt_balance() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    test_utils::process_test_delay(&config, "TerminalEBTBalance");

    // request object
    let mut request = blockchyp::BalanceRequest{
        test: true,
        terminal_name: config.default_terminal_name.clone().unwrap_or_else(|| "Test Terminal".to_string()).to_string(),
        card_type: blockchyp::CardType::EBT,
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.balance(&mut request);

    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
    assert!(!response.remaining_balance.is_empty());
}
