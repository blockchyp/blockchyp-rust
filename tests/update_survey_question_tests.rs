// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.

mod test_utils;
use ::blockchyp::*;

#[test]
fn test_update_survey_question() {
    let config = test_utils::load_test_configuration();
    let client = config.new_test_client(Some(""));

    // request object
    let request = SurveyQuestion{
        ordinal: 1,
        question_text: "Would you shop here again?".to_string(),
        question_type: "yes_no".to_string(),
        ..Default::default()
    };
    println!("Request: {:?}", request);

    let (response, err) = client.update_survey_question(&request);
    assert!(err.is_none(), "err is not none: {:?}", err);

    println!("Response: {:?}", response);

    // response assertions
    assert!(response.success);
    assert_eq!(response.question_text, "Would you shop here again?");
    assert_eq!(response.question_type, "yes_no");
}
