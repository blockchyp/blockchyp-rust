// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_partner_commission_breakdown() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some("partner"));

    // request object
    let request = blockchyp::PartnerCommissionBreakdownRequest{
        test: true,
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.partner_commission_breakdown(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
}
