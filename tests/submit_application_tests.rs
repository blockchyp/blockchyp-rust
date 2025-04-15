// Copyright 2019-2025 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use blockchyp;

#[test]
fn test_submit_application() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some("partner"));

    // request object
    let request = blockchyp::SubmitApplicationRequest{
        test: true,
        invite_code: "asdf".to_string(),
        dba_name: "BlockChyp".to_string(),
        corporate_name: "BlockChyp Inc.".to_string(),
        web_site: "https://www.blockchyp.com".to_string(),
        tax_id_number: "123456789".to_string(),
        entity_type: "CORPORATION".to_string(),
        state_of_incorporation: "UT".to_string(),
        merchant_type: "RETAIL".to_string(),
        business_description: "Payment processing solutions".to_string(),
        years_in_business: "5".to_string(),
        business_phone_number: "5555551234".to_string(),
        physical_address: blockchyp::Address{
            address_1: "355 S 520 W".to_string(),
            city: "Lindon".to_string(),
            state_or_province: "UT".to_string(),
            postal_code: "84042".to_string(),
            country_code: "US".to_string(),
            ..Default::default()
        },
        mailing_address: blockchyp::Address{
            address_1: "355 S 520 W".to_string(),
            city: "Lindon".to_string(),
            state_or_province: "UT".to_string(),
            postal_code: "84042".to_string(),
            country_code: "US".to_string(),
            ..Default::default()
        },
        contact_first_name: "John".to_string(),
        contact_last_name: "Doe".to_string(),
        contact_phone_number: "5555555678".to_string(),
        contact_email: "john.doe@example.com".to_string(),
        contact_title: "CEO".to_string(),
        contact_tax_id_number: "987654321".to_string(),
        contact_dob: "1980-01-01".to_string(),
        contact_dl_number: "D1234567".to_string(),
        contact_dl_state_or_province: "NY".to_string(),
        contact_dl_expiration: "2025-12-31".to_string(),
        contact_home_address: blockchyp::Address{
            address_1: "355 S 520 W".to_string(),
            city: "Lindon".to_string(),
            state_or_province: "UT".to_string(),
            postal_code: "84042".to_string(),
            country_code: "US".to_string(),
            ..Default::default()
        },
        contact_role: "OWNER".to_string(),
        owners: Some(vec![
            blockchyp::Owner{
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                job_title: "CEO".to_string(),
                tax_id_number: "876543210".to_string(),
                phone_number: "5555559876".to_string(),
                dob: "1981-02-02".to_string(),
                ownership: "50".to_string(),
                email: "john.doe@example.com".to_string(),
                dl_number: "D7654321".to_string(),
                dl_state_or_province: "UT".to_string(),
                dl_expiration: "2024-12-31".to_string(),
                address: blockchyp::Address{
                    address_1: "355 S 520 W".to_string(),
                    city: "Lindon".to_string(),
                    state_or_province: "UT".to_string(),
                    postal_code: "84042".to_string(),
                    country_code: "US".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            },

        ]),
        manual_account: blockchyp::ApplicationAccount{
            name: "Business Checking".to_string(),
            bank: "Test Bank".to_string(),
            account_holder_name: "BlockChyp Inc.".to_string(),
            routing_number: "124001545".to_string(),
            account_number: "987654321".to_string(),
            ..Default::default()
        },
        average_transaction: "100.00".to_string(),
        high_transaction: "1000.00".to_string(),
        average_month: "10000.00".to_string(),
        high_month: "20000.00".to_string(),
        refund_policy: "30_DAYS".to_string(),
        refund_days: "30".to_string(),
        time_zone: "America/Denver".to_string(),
        batch_close_time: "23:59".to_string(),
        multiple_locations: "false".to_string(),
        ebt_requested: "false".to_string(),
        ecommerce: "true".to_string(),
        card_present_percentage: "70".to_string(),
        phone_order_percentage: "10".to_string(),
        ecom_percentage: "20".to_string(),
        signer_name: "John Doe".to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.submit_application(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
}
