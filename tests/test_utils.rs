use chrono::Duration;
use serde::Deserialize;
use std::collections::HashMap;
use std::{env, path::Path};
use uuid::Uuid;

const DEFAULT_CONFIG_FILE: &str = "sdk-itest-config.json";
const DEFAULT_CONFIG_DIR: &str = "blockchyp";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TestConfiguration {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "bearerToken")]
    pub bearer_token: String,
    #[serde(rename = "signingKey")]
    pub signing_key: String,
    #[serde(rename = "defaultTerminalName")]
    pub default_terminal_name: Option<String>,
    #[serde(rename = "gatewayHost")]
    pub gateway_host: String,
    #[serde(rename = "dashboardHost")]
    pub dashboard_host: String,
    #[serde(rename = "testGatewayHost")]
    pub test_gateway_host: String,
    #[serde(rename = "defaultTerminalAddress")]
    pub default_terminal_address: Option<String>,
    pub profiles: Option<HashMap<String, ProfileCredentials>>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileCredentials {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "bearerToken")]
    pub bearer_token: String,
    #[serde(rename = "signingKey")]
    pub signing_key: String,
}

impl Default for TestConfiguration {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            bearer_token: String::new(),
            signing_key: String::new(),
            default_terminal_name: None,
            gateway_host: String::new(),
            dashboard_host: String::new(),
            test_gateway_host: String::new(),
            default_terminal_address: None,
            profiles: None,
        }
    }
}

pub fn load_test_configuration() -> TestConfiguration {
    let config_home = match env::var("XDG_CONFIG_HOME") {
        Ok(val) => val,
        Err(_) => match env::var("HOME") {
            Ok(home) => format!("{}/.config", home),
            Err(_) => String::new(),
        },
    };

    let file_name = if cfg!(windows) {
        Path::new(&env::var("USERPROFILE").unwrap())
            .join(DEFAULT_CONFIG_DIR)
            .join(DEFAULT_CONFIG_FILE)
    } else {
        Path::new(&config_home)
            .join(DEFAULT_CONFIG_DIR)
            .join(DEFAULT_CONFIG_FILE)
    };

    let contents = match std::fs::read_to_string(&file_name) {
        Ok(contents) => contents,
        Err(err) => panic!("Error reading configuration file: {}", err),
    };

    match serde_json::from_str::<TestConfiguration>(&contents) {
        Ok(config) => config,
        Err(err) => panic!("Error deserializing configuration: {}", err),
    }
}

impl TestConfiguration {
    pub fn new_test_client(&self, profile: Option<&str>) -> blockchyp::Client {
        let mut creds = blockchyp::APICredentials {
            api_key: self.api_key.clone(),
            bearer_token: self.bearer_token.clone(),
            signing_key: self.signing_key.clone(),
        };

        if let Some(alt_creds) = self.profiles.as_ref().and_then(|profiles| {
            profiles
                .iter()
                .find(|(key, _)| key == &profile.unwrap_or_default())
                .map(|(_, creds)| creds)
        }) {
            creds.api_key = alt_creds.api_key.clone();
            creds.bearer_token = alt_creds.bearer_token.clone();
            creds.signing_key = alt_creds.signing_key.clone();
        }

        let mut client = blockchyp::Client::new(creds);
        client.https = false;
        client.gateway_host = self.gateway_host.clone();
        client.dashboard_host = self.dashboard_host.clone();
        client.test_gateway_host = self.test_gateway_host.clone();

        client
    }
}

#[allow(dead_code)]
pub fn process_test_delay(config: &TestConfiguration, test_name: &str) {
    match env::var("BC_TEST_DELAY") {
        Ok(delay) => {
            if let Ok(test_delay_int) = delay.parse::<u64>() {
                let mut message_request = blockchyp::MessageRequest {
                    terminal_name: config.default_terminal_name.clone().unwrap_or_default(),
                    test: true,
                    message: format!(
                        "Running Test {} in {} seconds...",
                        test_name, test_delay_int
                    ),
                    ..Default::default()
                };

                let delay_client = config.new_test_client(None);
                let (_, err) = delay_client.message(&mut message_request);
                if let Some(err) = err {
                    println!("Error sending test delay message: {}", err);
                }

                let sleep_duration = Duration::seconds(test_delay_int as i64).to_std().unwrap();
                std::thread::sleep(sleep_duration);
            } else {
                panic!("Error parsing test delay to u64");
            }
        }
        Err(_) => println!("BC_TEST_DELAY is not set"),
    }
}

#[allow(dead_code)]
pub fn random_id() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}
