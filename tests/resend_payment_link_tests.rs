// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_resend_payment_link() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    // setup request object
    let setup_request = blockchyp::PaymentLinkRequest{
        amount: "199.99".to_string(),
        description: "Widget".to_string(),
        subject: "Widget invoice".to_string(),
        transaction: Some(blockchyp::TransactionDisplayTransaction{
            subtotal: "195.00".to_string(),
            tax: "4.99".to_string(),
            total: "199.99".to_string(),
            items: Some(vec![
                blockchyp::TransactionDisplayItem{
                    description: "Widget".to_string(),
                    price: "195.00".to_string(),
                    quantity: 1.0,
                    ..Default::default()
                },

            ]),
            ..Default::default()
        }),
        auto_send: true,
        customer: blockchyp::Customer{
            customer_ref: "Customer reference string".to_string(),
            first_name: "FirstName".to_string(),
            last_name: "LastName".to_string(),
            company_name: "Company Name".to_string(),
            email_address: "notifications@blockchypteam.m8r.co".to_string(),
            sms_number: "(123) 123-1231".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    println!("Setup Request: {:?}", setup_request);

    let (setup_response, err) = client.send_payment_link(&setup_request);

    assert!(err.is_none());

    println!("Setup Response: {:?}", setup_response);

    // request object
    let request = blockchyp::ResendPaymentLinkRequest{
        test: true,
        link_code: setup_response.link_code.to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.resend_payment_link(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
}
