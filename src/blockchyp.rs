// Copyright 2019-2024 BlockChyp, Inc. All rights reserved. Use of this code
// is governed by a license that can be found in the LICENSE file.
//
// This file was generated automatically by the BlockChyp SDK Generator.
// Changes to this file will be lost every time the code is regenerated.
use crate::models::*;
use std::{fs, thread};
use std::path::Path;
use std::sync::mpsc;
use std::error::Error;
use std::io::{Read, Write};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use reqwest::blocking::{Client as HttpClient, ClientBuilder, Request, RequestBuilder, Response};
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use reqwest::header::{HeaderMap, HeaderValue};
use aes::cipher::generic_array::GenericArray;
use rand::{RngCore, Rng, thread_rng};
use serde::{Deserialize, Serialize};
use reqwest::{Method, StatusCode};
use form_urlencoded::Serializer;
use serde::de::DeserializeOwned;
use reqwest::tls::Certificate;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use serde_json::Value;
use thiserror::Error;
use aes::Aes128;
use hex;


// Default client configuration constants.
const DEFAULT_GATEWAY_HOST: &str = "https://api.blockchyp.com";
const DEFAULT_DASHBOARD_HOST: &str = "https://dashboard.blockchyp.com";
const DEFAULT_TEST_GATEWAY_HOST: &str = "https://test.blockchyp.com";
const DEFAULT_HTTPS: bool = true;
const DEFAULT_ROUTE_CACHE_TTL: Duration = Duration::from_secs(60 * 60);
const DEFAULT_GATEWAY_TIMEOUT: Duration = Duration::from_secs(20);
const DEFAULT_TERMINAL_TIMEOUT: Duration = Duration::from_secs(2 * 60);
const CERTIFICATE_PATH: &str = "src/assets/BlockChyp.crt";
const USER_AGENT_STR: &str = concat!("BlockChyp-Rust/", env!("CARGO_PKG_VERSION"));
const AGENT_HEADER: &str = "X-Requested-With";

// Clientside response constants.
pub const RESPONSE_UNKNOWN_TERMINAL: &str  = "Unknown Terminal";
pub const RESPONSE_TIMED_OUT: &str  = "Request Timed Out";

// ErrUnknownTerminal is returned when there is no route to a given terminal.
#[derive(Debug, Error)]
#[error("unknown terminal")]
struct ErrUnknownTerminal;

// ErrNoChange is returned when a route refresh does not produce a new route.
#[derive(Debug, Error)]
#[error("route unchanged")]
struct ErrNoChange;

#[derive(Debug, Error)]
#[error("Invalid signature format")]
struct InvalidSignatureFormatError;

// HighClockDriftError is returned when there is high clock drift detected.
#[derive(Debug, Error)]
#[error("high clock drift, reset time on client")]
struct HighClockDriftError;

// Define the ConsumeResponseError using thiserror
#[derive(Debug, Error)]
pub enum ConsumeResponseError {
    #[error("Received an error response from the server: {0}")]
    ServerError(String),

    #[error("Failed to parse the response body: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Request failed with status: {0}")]
    StatusError(String),
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct TerminalRouteResponse {
    #[serde(flatten)]
    pub terminal_route: TerminalRoute,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct APIRequestHeaders {
    pub timestamp: String,
    pub nonce: String,
    pub bearer_token: String,
    pub api_key: String,
    pub signature: String,
}

// Models route information for a payment terminal.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalRoute {
    exists: bool,
    #[serde(rename = "terminalName")]
    terminal_name: String,
    #[serde(rename = "ipAddress")]
    ip_address: String,
    #[serde(rename = "cloudRelayEnabled")]
    cloud_relay_enabled: bool,
    #[serde(rename = "transientCredentials")]
    transient_credentials: Option<APICredentials>,
    #[serde(rename = "publicKey")]
    public_key: Option<String>,
    #[serde(rename = "rawKey")]
    raw_key: Option<RawPublicKey>,
    timestamp: Option<DateTime<Utc>>,
    https: bool,
}

// Models the primitive form of an ECC public key. A little simpler than X509, ASN and the usual nonsense.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawPublicKey {
    curve: String,
    x: String,
	#[serde(rename = "Y")]
    y: String,
}

// Models offline route cache information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteCache {
    routes: HashMap<String, RouteCacheEntry>,
}

// Models a route cache entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteCacheEntry {
    ttl: DateTime<Utc>,
    route: TerminalRoute,
}

fn build_gateway_http_client() -> Result<HttpClient, Box<dyn Error>> {
    Ok(HttpClient::builder()
        .timeout(DEFAULT_GATEWAY_TIMEOUT)
        .user_agent(USER_AGENT_STR)
        .build()?)
}

// Function to build terminal HTTP client
fn build_terminal_http_client() -> Result<HttpClient, Box<dyn Error>> {
    let cert_content = fs::read(CERTIFICATE_PATH)?;
    let cert = Certificate::from_pem(&cert_content)?;

    Ok(ClientBuilder::new()
        .timeout(DEFAULT_TERMINAL_TIMEOUT)
        .add_root_certificate(cert)
        .user_agent(USER_AGENT_STR)
        .build()?)
}

// Client is the main interface used by application developers.
#[derive(Debug)]
pub struct Client {
    pub credentials: APICredentials,
    pub gateway_host: String,
    pub dashboard_host: String,
    pub test_gateway_host: String,
    pub https: bool,
    pub route_cache: String,

    pub gateway_timeout: Duration,
    pub terminal_timeout: Duration,

    route_cache_ttl: Duration,
    gateway_http_client: HttpClient,
    terminal_http_client: HttpClient,
    route_cache_map: RouteCache,

    pub log_requests: bool,
}

impl Client {
    pub fn new(creds: APICredentials) -> Self {
        let gateway_http_client = match build_gateway_http_client() {
            Ok(client) => client,
            Err(err) => {
                eprintln!("Error building gateway HTTP client: {}", err);
                HttpClient::new()
            }
        };

        let terminal_http_client = match build_terminal_http_client() {
            Ok(client) => client,
            Err(err) => {
                eprintln!("Error building terminal HTTP client: {}", err);
                HttpClient::new()
            }
        };

        Client {
            credentials: creds,
            gateway_host: DEFAULT_GATEWAY_HOST.to_string(),
            dashboard_host: DEFAULT_DASHBOARD_HOST.to_string(),
            test_gateway_host: DEFAULT_TEST_GATEWAY_HOST.to_string(),
            https: DEFAULT_HTTPS,
            route_cache: std::env::temp_dir().join(".blockchyp_routes").to_str().unwrap().to_string(),

            gateway_timeout: DEFAULT_GATEWAY_TIMEOUT,
            terminal_timeout: DEFAULT_TERMINAL_TIMEOUT,

            route_cache_ttl: DEFAULT_ROUTE_CACHE_TTL,
            gateway_http_client,
            terminal_http_client,

            log_requests: false,
            route_cache_map: RouteCache{routes: HashMap::new()},
        }
    }

    /// Tests connectivity with a payment terminal.
	pub fn ping(&self, request: &mut PingRequest) -> (PingResponse, Option<Box<dyn Error>>) {
		let mut response = PingResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/terminal-test", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalPingRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/test", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/terminal-test", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Executes a standard direct preauth and capture.
	pub fn charge(&self, request: &mut AuthorizationRequest) -> (AuthorizationResponse, Option<Box<dyn Error>>) {
		let mut response = AuthorizationResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/charge", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalAuthorizationRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/charge", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/charge", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Executes a preauthorization intended to be captured later.
	pub fn preauth(&self, request: &mut AuthorizationRequest) -> (AuthorizationResponse, Option<Box<dyn Error>>) {
		let mut response = AuthorizationResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/preauth", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalAuthorizationRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/preauth", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/preauth", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Executes a refund.
	pub fn refund(&self, request: &mut RefundRequest) -> (AuthorizationResponse, Option<Box<dyn Error>>) {
		let mut response = AuthorizationResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/refund", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalRefundRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/refund", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/refund", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Adds a new payment method to the token vault.
	pub fn enroll(&self, request: &mut EnrollRequest) -> (EnrollResponse, Option<Box<dyn Error>>) {
		let mut response = EnrollResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/enroll", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalEnrollRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/enroll", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/enroll", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Activates or recharges a gift card.
	pub fn gift_activate(&self, request: &mut GiftActivateRequest) -> (GiftActivateResponse, Option<Box<dyn Error>>) {
		let mut response = GiftActivateResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/gift-activate", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalGiftActivateRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/gift-activate", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/gift-activate", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Checks the remaining balance on a payment method.
	pub fn balance(&self, request: &mut BalanceRequest) -> (BalanceResponse, Option<Box<dyn Error>>) {
		let mut response = BalanceResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/balance", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalBalanceRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/balance", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/balance", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Clears the line item display and any in progress transaction.
	pub fn clear(&self, request: &mut ClearTerminalRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/terminal-clear", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalClearTerminalRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/clear", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/terminal-clear", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Returns the current status of a terminal.
	pub fn terminal_status(&self, request: &mut TerminalStatusRequest) -> (TerminalStatusResponse, Option<Box<dyn Error>>) {
		let mut response = TerminalStatusResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/terminal-status", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalTerminalStatusRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/terminal-status", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/terminal-status", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Prompts the user to accept terms and conditions.
	pub fn terms_and_conditions(&self, request: &mut TermsAndConditionsRequest) -> (TermsAndConditionsResponse, Option<Box<dyn Error>>) {
		let mut response = TermsAndConditionsResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/terminal-tc", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalTermsAndConditionsRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/tc", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/terminal-tc", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Captures and returns a signature.
	pub fn capture_signature(&self, request: &mut CaptureSignatureRequest) -> (CaptureSignatureResponse, Option<Box<dyn Error>>) {
		let mut response = CaptureSignatureResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/capture-signature", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalCaptureSignatureRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/capture-signature", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/capture-signature", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Displays a new transaction on the terminal.
	pub fn new_transaction_display(&self, request: &mut TransactionDisplayRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/terminal-txdisplay", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalTransactionDisplayRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/txdisplay", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/terminal-txdisplay", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Appends items to an existing transaction display. Subtotal, Tax, and Total are
    /// overwritten by the request. Items with the same description are combined into
    /// groups.
	pub fn update_transaction_display(&self, request: &mut TransactionDisplayRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/terminal-txdisplay", "PUT", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalTransactionDisplayRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/txdisplay", "PUT", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/terminal-txdisplay", "PUT", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Displays a short message on the terminal.
	pub fn message(&self, request: &mut MessageRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/message", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalMessageRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/message", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/message", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Asks the consumer a yes/no question.
	pub fn boolean_prompt(&self, request: &mut BooleanPromptRequest) -> (BooleanPromptResponse, Option<Box<dyn Error>>) {
		let mut response = BooleanPromptResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/boolean-prompt", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalBooleanPromptRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/boolean-prompt", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/boolean-prompt", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Asks the consumer a text based question.
	pub fn text_prompt(&self, request: &mut TextPromptRequest) -> (TextPromptResponse, Option<Box<dyn Error>>) {
		let mut response = TextPromptResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/text-prompt", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalTextPromptRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/text-prompt", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/text-prompt", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Returns a list of queued transactions on a terminal.
	pub fn list_queued_transactions(&self, request: &mut ListQueuedTransactionsRequest) -> (ListQueuedTransactionsResponse, Option<Box<dyn Error>>) {
		let mut response = ListQueuedTransactionsResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/queue/list", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalListQueuedTransactionsRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/queue/list", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/queue/list", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Deletes a queued transaction from the terminal.
	pub fn delete_queued_transaction(&self, request: &mut DeleteQueuedTransactionRequest) -> (DeleteQueuedTransactionResponse, Option<Box<dyn Error>>) {
		let mut response = DeleteQueuedTransactionResponse::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/queue/delete", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalDeleteQueuedTransactionRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/queue/delete", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/queue/delete", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}

    /// Reboot a payment terminal.
	pub fn reboot(&self, request: &mut PingRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err: Result<(), Box<dyn Error>>;

		if let Err(e) = self.populate_signature_options(request) {
            return (response, Some(e));
        }

		if !request.terminal_name.is_empty() {
            match self.resolve_terminal_route(&request.terminal_name) {
                Ok(route) => {
                    if route.cloud_relay_enabled {
                        response_err = self.relay_request("/api/terminal-reboot", "POST", &request, &mut response, request.test, Some(request.timeout));
                    } else {
                        let auth_request = TerminalPingRequest {
                            api_credentials: route.transient_credentials.clone().unwrap_or_default(),
                            request: request.clone(),
                        };
                        response_err = self.terminal_request(route, "/api/reboot", "POST", &auth_request, &mut response, Some(request.timeout));
                    }
                }
                Err(e) => {
                    if e.downcast_ref::<ErrUnknownTerminal>().is_some() {
           	 		    response.response_description = RESPONSE_UNKNOWN_TERMINAL.to_string();
           	 		    return (response, Some(e))
           	 		} else {
           	 		    return (response, Some(e))
           	 		}
                }
            }
        } else {
			response_err = self.gateway_request("/api/terminal-reboot", "POST", request, &mut response, request.test, Some(request.timeout));
		}

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        if let Err(e) = self.handle_signature(&request, &mut response) {
            return (response, Some(e));
        }

		(response, err)
	}


    /// Returns routing and location data for a payment terminal.
	pub fn locate(&self, request: &LocateRequest) -> (LocateResponse, Option<Box<dyn Error>>) {
		let mut response = LocateResponse::default();
		let response_err = self.gateway_request("/api/terminal-locate", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Captures a preauthorization.
	pub fn capture(&self, request: &CaptureRequest) -> (CaptureResponse, Option<Box<dyn Error>>) {
		let mut response = CaptureResponse::default();
		let response_err = self.gateway_request("/api/capture", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Discards a previous transaction.
	pub fn void(&self, request: &VoidRequest) -> (VoidResponse, Option<Box<dyn Error>>) {
		let mut response = VoidResponse::default();
		let response_err = self.gateway_request("/api/void", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Executes a manual time out reversal.
    ///
    /// We love time out reversals. Don't be afraid to use them whenever a request to a
    /// BlockChyp terminal times out. You have up to two minutes to reverse any
    /// transaction. The only caveat is that you must assign transactionRef values when
    /// you build the original request. Otherwise, we have no real way of knowing which
    /// transaction you're trying to reverse because we may not have assigned it an id yet.
    /// And if we did assign it an id, you wouldn't know what it is because your request to the
    /// terminal timed out before you got a response.
	pub fn reverse(&self, request: &AuthorizationRequest) -> (AuthorizationResponse, Option<Box<dyn Error>>) {
		let mut response = AuthorizationResponse::default();
		let response_err = self.gateway_request("/api/reverse", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Closes the current credit card batch.
	pub fn close_batch(&self, request: &CloseBatchRequest) -> (CloseBatchResponse, Option<Box<dyn Error>>) {
		let mut response = CloseBatchResponse::default();
		let response_err = self.gateway_request("/api/close-batch", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Creates and send a payment link to a customer.
	pub fn send_payment_link(&self, request: &PaymentLinkRequest) -> (PaymentLinkResponse, Option<Box<dyn Error>>) {
		let mut response = PaymentLinkResponse::default();
		let response_err = self.gateway_request("/api/send-payment-link", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Resends payment link.
	pub fn resend_payment_link(&self, request: &ResendPaymentLinkRequest) -> (ResendPaymentLinkResponse, Option<Box<dyn Error>>) {
		let mut response = ResendPaymentLinkResponse::default();
		let response_err = self.gateway_request("/api/resend-payment-link", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Cancels a payment link.
	pub fn cancel_payment_link(&self, request: &CancelPaymentLinkRequest) -> (CancelPaymentLinkResponse, Option<Box<dyn Error>>) {
		let mut response = CancelPaymentLinkResponse::default();
		let response_err = self.gateway_request("/api/cancel-payment-link", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Retrieves the status of a payment link.
	pub fn payment_link_status(&self, request: &PaymentLinkStatusRequest) -> (PaymentLinkStatusResponse, Option<Box<dyn Error>>) {
		let mut response = PaymentLinkStatusResponse::default();
		let response_err = self.gateway_request("/api/payment-link-status", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Retrieves the current status of a transaction.
	pub fn transaction_status(&self, request: &TransactionStatusRequest) -> (AuthorizationResponse, Option<Box<dyn Error>>) {
		let mut response = AuthorizationResponse::default();
		let response_err = self.gateway_request("/api/tx-status", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Updates or creates a customer record.
	pub fn update_customer(&self, request: &UpdateCustomerRequest) -> (CustomerResponse, Option<Box<dyn Error>>) {
		let mut response = CustomerResponse::default();
		let response_err = self.gateway_request("/api/update-customer", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Retrieves a customer by id.
	pub fn customer(&self, request: &CustomerRequest) -> (CustomerResponse, Option<Box<dyn Error>>) {
		let mut response = CustomerResponse::default();
		let response_err = self.gateway_request("/api/customer", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Searches the customer database.
	pub fn customer_search(&self, request: &CustomerSearchRequest) -> (CustomerSearchResponse, Option<Box<dyn Error>>) {
		let mut response = CustomerSearchResponse::default();
		let response_err = self.gateway_request("/api/customer-search", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Calculates the discount for actual cash transactions.
	pub fn cash_discount(&self, request: &CashDiscountRequest) -> (CashDiscountResponse, Option<Box<dyn Error>>) {
		let mut response = CashDiscountResponse::default();
		let response_err = self.gateway_request("/api/cash-discount", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Returns the batch history for a merchant.
	pub fn batch_history(&self, request: &BatchHistoryRequest) -> (BatchHistoryResponse, Option<Box<dyn Error>>) {
		let mut response = BatchHistoryResponse::default();
		let response_err = self.gateway_request("/api/batch-history", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Returns the batch details for a single batch.
	pub fn batch_details(&self, request: &BatchDetailsRequest) -> (BatchDetailsResponse, Option<Box<dyn Error>>) {
		let mut response = BatchDetailsResponse::default();
		let response_err = self.gateway_request("/api/batch-details", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Returns the transaction history for a merchant.
	pub fn transaction_history(&self, request: &TransactionHistoryRequest) -> (TransactionHistoryResponse, Option<Box<dyn Error>>) {
		let mut response = TransactionHistoryResponse::default();
		let response_err = self.gateway_request("/api/tx-history", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Returns pricing policy for a merchant.
	pub fn pricing_policy(&self, request: &PricingPolicyRequest) -> (PricingPolicyResponse, Option<Box<dyn Error>>) {
		let mut response = PricingPolicyResponse::default();
		let response_err = self.gateway_request("/api/read-pricing-policy", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Returns a list of partner statements.
	pub fn partner_statements(&self, request: &PartnerStatementListRequest) -> (PartnerStatementListResponse, Option<Box<dyn Error>>) {
		let mut response = PartnerStatementListResponse::default();
		let response_err = self.gateway_request("/api/partner-statement-list", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Returns detail for a single partner statement.
	pub fn partner_statement_detail(&self, request: &PartnerStatementDetailRequest) -> (PartnerStatementDetailResponse, Option<Box<dyn Error>>) {
		let mut response = PartnerStatementDetailResponse::default();
		let response_err = self.gateway_request("/api/partner-statement-detail", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Returns a list of merchant invoices.
	pub fn merchant_invoices(&self, request: &MerchantInvoiceListRequest) -> (MerchantInvoiceListResponse, Option<Box<dyn Error>>) {
		let mut response = MerchantInvoiceListResponse::default();
		let response_err = self.gateway_request("/api/merchant-invoice-list", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Returns detail for a single merchant-invoice statement.
	pub fn merchant_invoice_detail(&self, request: &MerchantInvoiceDetailRequest) -> (MerchantInvoiceDetailResponse, Option<Box<dyn Error>>) {
		let mut response = MerchantInvoiceDetailResponse::default();
		let response_err = self.gateway_request("/api/merchant-invoice-detail", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Returns low level details for how partner commissions were calculated for a
    /// specific merchant statement.
	pub fn partner_commission_breakdown(&self, request: &PartnerCommissionBreakdownRequest) -> (PartnerCommissionBreakdownResponse, Option<Box<dyn Error>>) {
		let mut response = PartnerCommissionBreakdownResponse::default();
		let response_err = self.gateway_request("/api/partner-commission-breakdown", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Returns profile information for a merchant.
	pub fn merchant_profile(&self, request: &MerchantProfileRequest) -> (MerchantProfileResponse, Option<Box<dyn Error>>) {
		let mut response = MerchantProfileResponse::default();
		let response_err = self.gateway_request("/api/public-merchant-profile", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Deletes a customer record.
	pub fn delete_customer(&self, request: &DeleteCustomerRequest) -> (DeleteCustomerResponse, Option<Box<dyn Error>>) {
		let mut response = DeleteCustomerResponse::default();
		let response_err = self.gateway_request(&format!("/api/customer/{}", request.customer_id), "DELETE", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Retrieves payment token metadata.
	pub fn token_metadata(&self, request: &TokenMetadataRequest) -> (TokenMetadataResponse, Option<Box<dyn Error>>) {
		let mut response = TokenMetadataResponse::default();
		let response_err = self.gateway_request(&format!("/api/token/{}", request.token), "GET", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Links a token to a customer record.
	pub fn link_token(&self, request: &LinkTokenRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.gateway_request("/api/link-token", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Removes a link between a customer and a token.
	pub fn unlink_token(&self, request: &UnlinkTokenRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.gateway_request("/api/unlink-token", "POST", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}
    /// Deletes a payment token.
	pub fn delete_token(&self, request: &DeleteTokenRequest) -> (DeleteTokenResponse, Option<Box<dyn Error>>) {
		let mut response = DeleteTokenResponse::default();
		let response_err = self.gateway_request(&format!("/api/token/{}", request.token), "DELETE", request, &mut response, request.test, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

		(response, err)
	}

    /// Generates and returns api credentials for a given merchant.
	pub fn merchant_credential_generation(&self, request: &MerchantCredentialGenerationRequest) -> (MerchantCredentialGenerationResponse, Option<Box<dyn Error>>) {
		let mut response = MerchantCredentialGenerationResponse::default();
		let response_err = self.dashboard_request("/api/generate-merchant-creds", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Adds a test merchant account.
	pub fn get_merchants(&self, request: &GetMerchantsRequest) -> (GetMerchantsResponse, Option<Box<dyn Error>>) {
		let mut response = GetMerchantsResponse::default();
		let response_err = self.dashboard_request("/api/get-merchants", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Adds or updates a merchant account. Can be used to create or update test merchants.
    /// Only gateway partners may create new live merchants.
	pub fn update_merchant(&self, request: &MerchantProfile) -> (MerchantProfileResponse, Option<Box<dyn Error>>) {
		let mut response = MerchantProfileResponse::default();
		let response_err = self.dashboard_request("/api/update-merchant", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// List all active users and pending invites for a merchant account.
	pub fn merchant_users(&self, request: &MerchantProfileRequest) -> (MerchantUsersResponse, Option<Box<dyn Error>>) {
		let mut response = MerchantUsersResponse::default();
		let response_err = self.dashboard_request("/api/merchant-users", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Invites a user to join a merchant account.
	pub fn invite_merchant_user(&self, request: &InviteMerchantUserRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.dashboard_request("/api/invite-merchant-user", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Adds a test merchant account.
	pub fn add_test_merchant(&self, request: &AddTestMerchantRequest) -> (MerchantProfileResponse, Option<Box<dyn Error>>) {
		let mut response = MerchantProfileResponse::default();
		let response_err = self.dashboard_request("/api/add-test-merchant", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Deletes a test merchant account. Supports partner scoped API credentials only.
    /// Live merchant accounts cannot be deleted.
	pub fn delete_test_merchant(&self, request: &MerchantProfileRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.dashboard_request(&format!("/api/test-merchant/{}", request.merchant_id), "DELETE", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// List all merchant platforms configured for a gateway merchant.
	pub fn merchant_platforms(&self, request: &MerchantProfileRequest) -> (MerchantPlatformsResponse, Option<Box<dyn Error>>) {
		let mut response = MerchantPlatformsResponse::default();
		let response_err = self.dashboard_request(&format!("/api/plugin-configs/{}", request.merchant_id), "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// List all merchant platforms configured for a gateway merchant.
	pub fn update_merchant_platforms(&self, request: &MerchantPlatform) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.dashboard_request("/api/plugin-configs", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Deletes a boarding platform configuration.
	pub fn delete_merchant_platforms(&self, request: &MerchantPlatformRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.dashboard_request(&format!("/api/plugin-config/{}", request.platform_id), "DELETE", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns all terminals associated with the merchant account.
	pub fn terminals(&self, request: &TerminalProfileRequest) -> (TerminalProfileResponse, Option<Box<dyn Error>>) {
		let mut response = TerminalProfileResponse::default();
		let response_err = self.dashboard_request("/api/terminals", "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Deactivates a terminal.
	pub fn deactivate_terminal(&self, request: &TerminalDeactivationRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.dashboard_request(&format!("/api/terminal/{}", request.terminal_id), "DELETE", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Activates a terminal.
	pub fn activate_terminal(&self, request: &TerminalActivationRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.dashboard_request("/api/terminal-activate", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns a list of terms and conditions templates associated with a merchant
    /// account.
	pub fn tc_templates(&self, request: &TermsAndConditionsTemplateRequest) -> (TermsAndConditionsTemplateResponse, Option<Box<dyn Error>>) {
		let mut response = TermsAndConditionsTemplateResponse::default();
		let response_err = self.dashboard_request("/api/tc-templates", "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns a single terms and conditions template.
	pub fn tc_template(&self, request: &TermsAndConditionsTemplateRequest) -> (TermsAndConditionsTemplate, Option<Box<dyn Error>>) {
		let mut response = TermsAndConditionsTemplate::default();
		let response_err = self.dashboard_request(&format!("/api/tc-templates/{}", request.template_id), "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Updates or creates a terms and conditions template.
	pub fn tc_update_template(&self, request: &TermsAndConditionsTemplate) -> (TermsAndConditionsTemplate, Option<Box<dyn Error>>) {
		let mut response = TermsAndConditionsTemplate::default();
		let response_err = self.dashboard_request("/api/tc-templates", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Deletes a single terms and conditions template.
	pub fn tc_delete_template(&self, request: &TermsAndConditionsTemplateRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.dashboard_request(&format!("/api/tc-templates/{}", request.template_id), "DELETE", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns up to 250 entries from the Terms and Conditions log.
	pub fn tc_log(&self, request: &TermsAndConditionsLogRequest) -> (TermsAndConditionsLogResponse, Option<Box<dyn Error>>) {
		let mut response = TermsAndConditionsLogResponse::default();
		let response_err = self.dashboard_request("/api/tc-log", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns a single detailed Terms and Conditions entry.
	pub fn tc_entry(&self, request: &TermsAndConditionsLogRequest) -> (TermsAndConditionsLogEntry, Option<Box<dyn Error>>) {
		let mut response = TermsAndConditionsLogEntry::default();
		let response_err = self.dashboard_request(&format!("/api/tc-entry/{}", request.log_entry_id), "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns all survey questions for a given merchant.
	pub fn survey_questions(&self, request: &SurveyQuestionRequest) -> (SurveyQuestionResponse, Option<Box<dyn Error>>) {
		let mut response = SurveyQuestionResponse::default();
		let response_err = self.dashboard_request("/api/survey-questions", "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns a single survey question with response data.
	pub fn survey_question(&self, request: &SurveyQuestionRequest) -> (SurveyQuestion, Option<Box<dyn Error>>) {
		let mut response = SurveyQuestion::default();
		let response_err = self.dashboard_request(&format!("/api/survey-questions/{}", request.question_id), "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Updates or creates a survey question.
	pub fn update_survey_question(&self, request: &SurveyQuestion) -> (SurveyQuestion, Option<Box<dyn Error>>) {
		let mut response = SurveyQuestion::default();
		let response_err = self.dashboard_request("/api/survey-questions", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Deletes a survey question.
	pub fn delete_survey_question(&self, request: &SurveyQuestionRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.dashboard_request(&format!("/api/survey-questions/{}", request.question_id), "DELETE", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns results for a single survey question.
	pub fn survey_results(&self, request: &SurveyResultsRequest) -> (SurveyQuestion, Option<Box<dyn Error>>) {
		let mut response = SurveyQuestion::default();
		let response_err = self.dashboard_request("/api/survey-results", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns the media library for a given partner, merchant, or organization.
	pub fn media(&self, request: &MediaRequest) -> (MediaLibraryResponse, Option<Box<dyn Error>>) {
		let mut response = MediaLibraryResponse::default();
		let response_err = self.dashboard_request("/api/media", "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Uploads a media asset to the media library.
	pub fn upload_media(&self, request: &UploadMetadata, reader: &mut dyn Read) -> (MediaMetadata, Option<Box<dyn Error>>) {
		let mut response = MediaMetadata::default();
		let response_err = self.dashboard_upload("/api/upload-media", request, reader, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Retrieves the current status of a file upload.
	pub fn upload_status(&self, request: &UploadStatusRequest) -> (UploadStatus, Option<Box<dyn Error>>) {
		let mut response = UploadStatus::default();
		let response_err = self.dashboard_request(&format!("/api/media-upload/{}", request.upload_id), "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns the media details for a single media asset.
	pub fn media_asset(&self, request: &MediaRequest) -> (MediaMetadata, Option<Box<dyn Error>>) {
		let mut response = MediaMetadata::default();
		let response_err = self.dashboard_request(&format!("/api/media/{}", request.media_id), "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Deletes a media asset.
	pub fn delete_media_asset(&self, request: &MediaRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.dashboard_request(&format!("/api/media/{}", request.media_id), "DELETE", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns a collection of slide shows.
	pub fn slide_shows(&self, request: &SlideShowRequest) -> (SlideShowResponse, Option<Box<dyn Error>>) {
		let mut response = SlideShowResponse::default();
		let response_err = self.dashboard_request("/api/slide-shows", "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns a single slide show with slides.
	pub fn slide_show(&self, request: &SlideShowRequest) -> (SlideShow, Option<Box<dyn Error>>) {
		let mut response = SlideShow::default();
		let response_err = self.dashboard_request(&format!("/api/slide-shows/{}", request.slide_show_id), "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Updates or creates a slide show.
	pub fn update_slide_show(&self, request: &SlideShow) -> (SlideShow, Option<Box<dyn Error>>) {
		let mut response = SlideShow::default();
		let response_err = self.dashboard_request("/api/slide-shows", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Deletes a single slide show.
	pub fn delete_slide_show(&self, request: &SlideShowRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.dashboard_request(&format!("/api/slide-shows/{}", request.slide_show_id), "DELETE", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Returns the terminal branding stack for a given set of API credentials.
	pub fn terminal_branding(&self, request: &BrandingAssetRequest) -> (BrandingAssetResponse, Option<Box<dyn Error>>) {
		let mut response = BrandingAssetResponse::default();
		let response_err = self.dashboard_request("/api/terminal-branding", "GET", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Updates a branding asset.
	pub fn update_branding_asset(&self, request: &BrandingAsset) -> (BrandingAsset, Option<Box<dyn Error>>) {
		let mut response = BrandingAsset::default();
		let response_err = self.dashboard_request("/api/terminal-branding", "POST", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}
    /// Deletes a branding asset.
	pub fn delete_branding_asset(&self, request: &BrandingAssetRequest) -> (Acknowledgement, Option<Box<dyn Error>>) {
		let mut response = Acknowledgement::default();
		let response_err = self.dashboard_request(&format!("/api/terminal-branding/{}", request.asset_id), "DELETE", request, &mut response, Some(request.timeout));

		let err = if let Err(e) = response_err {
            if let Some(reqwest_err) = e.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_timeout() {
                    response.response_description = RESPONSE_TIMED_OUT.to_string();
                } else {
                    response.response_description = e.to_string();
                }
            } else {
                response.response_description = e.to_string();
            }
            Some(e)
        } else {
            None
        };

        (response, err)
	}

    fn high_clock_diff(&self) -> bool {
        let mut response = HeartbeatResponse::default();
        if self.gateway_request("/api/heartbeat",Method::GET.as_str(),&(),&mut response,false,None,).is_err() {
            return false;
        }

        let dur = Utc::now() - response.timestamp;
        let dur_in_minutes = dur.num_minutes();

        dur_in_minutes.abs() > 10
    }

    fn consume_response<R: DeserializeOwned>(&self, resp: Response, response_entity: &mut R) -> Result<(), Box<dyn Error>> {
        let status = resp.status();
        let body = resp.text()?;
        let mut json_value: Value = serde_json::from_str(&body)?;

        if let Some(obj) = json_value.as_object_mut() {
            obj.entry("success").or_insert_with(|| Value::Bool(false));
            obj.entry("error").or_insert_with(|| Value::String(String::new()));
            obj.entry("responseDescription").or_insert_with(|| Value::String(String::new()));
        }

        if status != reqwest::StatusCode::OK {
            let ack: Result<Acknowledgement, _> = serde_json::from_value(json_value.clone());
            if let Ok(ack) = ack {
                if !ack.error.is_empty() {
                    return Err(Box::new(ConsumeResponseError::ServerError(ack.error)));
                }
            } else {
                return Err(Box::new(ConsumeResponseError::StatusError(status.to_string())));
            }
        }

        *response_entity = serde_json::from_value(json_value)?;
        Ok(())
    }

    fn gateway_request<T: Serialize, R: DeserializeOwned>(&self,path: &str,method: &str,request: &T, response: &mut R,test_tx: bool,request_timeout: Option<i32>,) -> Result<(), Box<dyn Error>> {
		let content = serde_json::to_string(request)?;
		let url = self.assemble_gateway_url(path, test_tx);
		let request_builder = self.gateway_http_client.request(reqwest::Method::from_bytes(method.as_bytes())?, url).body(content).headers(HeaderMap::new());
        let req = self.add_api_request_headers( request_builder, &self.credentials)?;

        if self.log_requests {
            let req_log = format!("GATEWAY REQUEST: {:?}", &req);
            let mut stderr = std::io::stderr();
            writeln!(stderr, "{}", req_log)?;
        }

        let (sender, receiver) = mpsc::channel();
        let client = self.gateway_http_client.clone();

        thread::spawn(move || {
            let response = client.execute(req);
            let _ = sender.send(response);
        });

        let start = Instant::now();
        let res = loop {
            if let Ok(response) = receiver.try_recv() {
                break response;
            }
            if start.elapsed() >= self.get_timeout(request_timeout, self.gateway_timeout) {
                return Err(RESPONSE_TIMED_OUT.into());
            }
            thread::sleep(Duration::from_millis(50));
        };

        let res = res?;
		if res.status() == StatusCode::FORBIDDEN && self.high_clock_diff() {
            return Err(Box::new(HighClockDriftError));
        }

        self.consume_response( res, response)?;
        Ok(())
    }

    fn dashboard_request<T: Serialize, R: DeserializeOwned>(&self, path: &str,method: &str,request: &T, response: &mut R,request_timeout: Option<i32>,) -> Result<(), Box<dyn Error>> {
        let content = serde_json::to_string(request)?;
        let url = self.assemble_dashboard_url(path);
		let request_builder = self.gateway_http_client.request(reqwest::Method::from_bytes(method.as_bytes())?, url).body(content).headers(HeaderMap::new());
        let req = self.add_api_request_headers( request_builder, &self.credentials)?;

        if self.log_requests {
            let req_log = format!("DASHBOARD REQUEST: {:?}", &req);
            let mut stderr = std::io::stderr();
            writeln!(stderr, "{}", req_log)?;
        }

        let (sender, receiver) = mpsc::channel();
        let client = self.gateway_http_client.clone();

        thread::spawn(move || {
            let response = client.execute(req);
            let _ = sender.send(response);
        });

        let start = Instant::now();
        let res = loop {
            if let Ok(response) = receiver.try_recv() {
                break response;
            }
            if start.elapsed() >= self.get_timeout(request_timeout, self.gateway_timeout) {
                return Err(RESPONSE_TIMED_OUT.into());
            }
            thread::sleep(Duration::from_millis(50));
        };

        let res = res?;
		if res.status() == StatusCode::FORBIDDEN && self.high_clock_diff() {
            return Err(Box::new(HighClockDriftError));
        }

        self.consume_response( res, response)?;
        Ok(())
    }

    fn dashboard_upload<R: DeserializeOwned>(&self, path: &str, request: &UploadMetadata, reader: &mut dyn Read, response: &mut R, request_timeout: Option<i32>,) -> Result<(), Box<dyn Error>> {
        let url = self.assemble_dashboard_url(path);
        let mut content = Vec::new();
        reader.read_to_end(&mut content)?;

		let mut request_builder = self.gateway_http_client.request(reqwest::Method::POST, url).body(content).headers(HeaderMap::new());

        request_builder = request_builder.header("X-File-Size", request.file_size.to_string());
        request_builder = request_builder.header("X-Upload-File-Name", request.file_name.to_string());
        request_builder = request_builder.header("X-Upload-ID", request.upload_id.to_string());

        let req = self.add_api_request_headers( request_builder, &self.credentials)?;

        if self.log_requests {
            let req_log = format!("DASHBOARD REQUEST: {:?}", &req);
            let mut stderr = std::io::stderr();
            writeln!(stderr, "{}", req_log)?;
        }

        let (sender, receiver) = mpsc::channel();
        let client = self.gateway_http_client.clone();


        thread::spawn(move || {
            let response = client.execute(req);
            let _ = sender.send(response);
        });

        let start = Instant::now();
        let res = loop {
            if let Ok(response) = receiver.try_recv() {
                break response;
            }
            if start.elapsed() >= self.get_timeout(request_timeout, self.gateway_timeout) {
                return Err(RESPONSE_TIMED_OUT.into());
            }
            thread::sleep(Duration::from_millis(50));
        };

        let res = res?;
		if res.status() == StatusCode::FORBIDDEN && self.high_clock_diff() {
            return Err(Box::new(HighClockDriftError));
        }

        self.consume_response(res, response)?;
        Ok(())
    }

    fn terminal_request<T: Serialize, R: DeserializeOwned>(&self,route: TerminalRoute,path: &str,method: &str,request: &T, response: &mut R,request_timeout: Option<i32>,) -> Result<(), Box<dyn Error>> {
        let content = serde_json::to_string(request)?;
		let url = self.assemble_terminal_url(&route, path);
		let request_builder = self.terminal_http_client.request(reqwest::Method::from_bytes(method.as_bytes())?, url).body(content).headers(HeaderMap::new());
        let req = self.add_api_request_headers( request_builder, &self.credentials)?;

        if self.log_requests {
            let req_log = format!("TERMINAL REQUEST: {:?}", &req);
            let mut stderr = std::io::stderr();
            writeln!(stderr, "{}", req_log)?;
        }

        let (sender, receiver) = mpsc::channel();
        let client = self.terminal_http_client.clone();

        thread::spawn(move || {
            let response = client.execute(req);
            let _ = sender.send(response);
        });

        let start = Instant::now();
        let http_response = loop {
            if let Ok(response) = receiver.try_recv() {
                break response;
            }
            if start.elapsed() >= self.get_timeout(request_timeout, self.terminal_timeout) {
                return Err(RESPONSE_TIMED_OUT.into());
            }
            thread::sleep(Duration::from_millis(50));
        };

		let res = match http_response {
			Ok(res) => res,
			Err(e) => {
				let r_route_result = self.refresh_route(route);
                if let Ok(Some(r_route)) = r_route_result {
                    self.route_cache_put(r_route.clone());
                    return self.terminal_request(r_route, path, method, request, response, request_timeout);
				} else {
					return Err(Box::new(e));
				}
			}
		};

        self.consume_response( res, response)?;
        Ok(())
    }

    fn refresh_route(&self, route: TerminalRoute) -> Result<Option<TerminalRoute>, Box<dyn Error>> {
        if let Some(res) = self.request_route_from_gateway(&route.terminal_name)? {
            if res.ip_address != route.ip_address {
                return Ok(Some(res));
            }
        }
        Err(Box::new(ErrNoChange))
    }

	fn request_route_from_gateway(&self, terminal_name: &str) -> Result<Option<TerminalRoute>, Box<dyn Error>> {
        let encoded_terminal_name = Serializer::new(String::new())
            .append_pair("terminal", terminal_name)
            .finish();
        let path = format!("/api/terminal-route?{}", encoded_terminal_name);

		let mut res = TerminalRouteResponse::default();
		self.gateway_request(&path, "GET", &(), &mut res, false, None)?;

        if res.success && !res.terminal_route.ip_address.is_empty() {
            let mut route = res.terminal_route;
            route.exists = true;
            route.https = true;
            Ok(Some(route))
        } else {
            Err(ErrUnknownTerminal.into())
        }
    }

    fn relay_request<T: Serialize, R: DeserializeOwned>(&self, path: &str, method: &str, request: &T, response: &mut R, test_tx: bool, request_timeout: Option<i32>) -> Result<(), Box<dyn Error>> {
        let timeout = Some(self.get_timeout(request_timeout, self.terminal_timeout).as_secs() as i32);
        self.gateway_request(path, method, request, response, test_tx, timeout)
    }

	fn resolve_terminal_route(&self, terminal_name: &str) -> Result<TerminalRoute, Box<dyn Error>> {
        if let Some(route) = self.route_cache_get(terminal_name, false) {
            return Ok(route);
        }

        // By-pass route cache lookup for IP addresses
        if terminal_name.matches('.').count() == 3 {
            let route = TerminalRoute {
				terminal_name: terminal_name.to_string(),
				ip_address: terminal_name.to_string(),
				transient_credentials: Some(self.credentials.clone()),
				exists: true,
				..Default::default()
			};
            return Ok(route);
        }

        let route_request = self.request_route_from_gateway(terminal_name)?;

        let route = match route_request {
			Some(route) => route,
            None => {
                if let Some(route) = self.route_cache_get(terminal_name, true) {
                    return Ok(route);
                }
                return Err(ErrUnknownTerminal.into()); 
            }
        };

        self.route_cache_put(route.clone());
        Ok(route)
    }

    fn route_cache_put(&self, route: TerminalRoute) {
        let combined_key = format!("{}{}", self.credentials.api_key, route.terminal_name);

        let mut new_cache_map = self.route_cache_map.clone();
        let mut cache_entry = RouteCacheEntry {
            ttl: Utc::now() + self.route_cache_ttl,
            route: route.clone(),
        };

        new_cache_map.routes.insert(combined_key, cache_entry.clone());

        self.update_offline_cache(&mut cache_entry);
    }

    fn route_cache_get(&self, terminal_name: &str, stale: bool) -> Option<TerminalRoute> {
        if let Some(route_entry) = self.route_cache_map.routes.get(terminal_name) {
            if route_entry.ttl > Utc::now() {
                return Some(route_entry.route.clone());
            } else {
                return None;
            }
        }

        if let Some(mut cache_entry) = self.read_from_offline_cache(terminal_name) {
            if !stale && cache_entry.ttl < Utc::now() {
                return None;
            }

            if let Some(mut transient_credentials) = cache_entry.route.transient_credentials {
                transient_credentials.api_key = self.decrypt(&transient_credentials.api_key);
                transient_credentials.bearer_token = self.decrypt(&transient_credentials.bearer_token);
                transient_credentials.signing_key = self.decrypt(&transient_credentials.signing_key);

                cache_entry.route.transient_credentials = Some(transient_credentials);
                return Some(cache_entry.route);
            } else {
                return Some(cache_entry.route);
            }
        }

        None
    }

    fn update_offline_cache(&self, cache_entry: &mut RouteCacheEntry) {
        let mut cache = self.read_offline_cache().unwrap_or(RouteCache {
            routes: HashMap::new(),
        });

        let mut routes = cache.routes.clone();

        if let Some(ref mut transient_credentials) = cache_entry.route.transient_credentials {
            transient_credentials.api_key = self.encrypt(&transient_credentials.api_key);
            transient_credentials.bearer_token = self.encrypt(&transient_credentials.bearer_token);
            transient_credentials.signing_key = self.encrypt(&transient_credentials.signing_key);
            cache_entry.route.transient_credentials = Some(transient_credentials.clone());
        } else {
            return
        }

        let key = &self.credentials.api_key;
        let terminal_name = &cache_entry.route.terminal_name;
        let route_key = format!("{}{}", key, terminal_name);

        routes.insert(route_key, (*cache_entry).clone());
        cache.routes = routes;

        let content_result = serde_json::to_string(&cache);
        let content = match content_result {
            Ok(json_string) => json_string,
            Err(err) => {
                eprintln!("Failed to serialize cache to JSON: {}", err);
                return;
            }
        };
        if let Err(err) = fs::write(&self.route_cache, content) {
            eprintln!("Error writing to route cache file: {}", err);
        }
    }

    fn read_from_offline_cache(&self, terminal_name: &str) -> Option<RouteCacheEntry> {
        if let Some(cache) = self.read_offline_cache() {
            if let Some(route) = cache.routes.get(&(self.credentials.api_key.clone() + terminal_name)) {
                return Some(route.clone());
            }
        }
        None
    }

    fn read_offline_cache(&self) -> Option<RouteCache> {
        if !Path::new(&self.route_cache).exists() {
            return None;
        }

        if let Ok(content) = fs::read_to_string(&self.route_cache) {
            if let Ok(cache) = serde_json::from_str(&content) {
                return Some(cache);
            }
        }
        None
    }

	fn encrypt(&self, value: &str) -> String {
        let key = self.derive_offline_key();
        self.encrypt_data(&key, value)
    }

    fn decrypt(&self, value: &str) -> String {
        let key = self.derive_offline_key();
        self.decrypt_data(&key, value)
    }

    fn derive_offline_key(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        let fixed_key = hex::decode("cb22789c9d5c344a10e0474f134db39e25eb3bbf5a1b1a5e89b507f15ea9519c")
            .unwrap_or_else(|err| {
                eprintln!("Decoding fixed key failed: {}", err);
                Vec::new()
            });
        hasher.update(&fixed_key);
        let dynamic_key = hex::decode(&self.credentials.signing_key)
            .unwrap_or_else(|err| {
                eprintln!("Decoding dynamic key failed: {}", err);
                Vec::new()
            });
        hasher.update(dynamic_key);
        hasher.finalize().to_vec()
    }

    fn encrypt_data(&self, key: &[u8], plain_text: &str) -> String {
        let key = &key[..16];
        let plain_bytes = self.pkcs7_pad(plain_text.as_bytes(), 16);

        let mut iv = [0u8; 16];
        let mut rng = thread_rng();
        rng.fill(&mut iv); // Generate random IV

        let cipher = Aes128::new(GenericArray::from_slice(key));

        let mut cipher_text = iv.to_vec();

        let mut prev_block = iv;

        for chunk in plain_bytes.chunks(16) {
            let mut block = [0u8; 16];
            for (i, &byte) in chunk.iter().enumerate() {
                block[i] = byte ^ prev_block[i];
            }

            let mut block_ga = GenericArray::clone_from_slice(&block);
            cipher.encrypt_block(&mut block_ga);

            cipher_text.extend_from_slice(&block_ga);
            prev_block.copy_from_slice(&block_ga);
        }

        hex::encode(&cipher_text)
    }

    fn decrypt_data(&self, key: &[u8], cipher_text: &str) -> String {
        let cipher_bytes = hex::decode(cipher_text).unwrap();

        let key = &key[..16];
        let iv = &cipher_bytes[..16];
        let cipher_data = &cipher_bytes[16..];

        let cipher = Aes128::new(GenericArray::from_slice(key));
        let mut plain_bytes = vec![0u8; cipher_data.len()];
        let mut prev_block = iv;

        for (i, chunk) in cipher_data.chunks(16).enumerate() {
            let mut block = GenericArray::clone_from_slice(chunk);
            cipher.decrypt_block(&mut block);

            for j in 0..16 {
                plain_bytes[i * 16 + j] = block[j] ^ prev_block[j];
            }

            prev_block = chunk;
        }

        String::from_utf8(self.pkcs7_unpad(&plain_bytes)).unwrap()
    }

    fn pkcs7_pad(&self, data: &[u8], block_size: usize) -> Vec<u8> {
        let padding_length = block_size - (data.len() % block_size);
        let mut padded_data = Vec::with_capacity(data.len() + padding_length);
        padded_data.extend_from_slice(data);
        padded_data.extend(std::iter::repeat(padding_length as u8).take(padding_length));
        padded_data
    }

    fn pkcs7_unpad(&self, data: &[u8]) -> Vec<u8> {
        let data_length = data.len();
        if data_length == 0 {
            return Vec::new();
        }
        let padding_length = *data.last().unwrap() as usize;

        if padding_length == 0 || padding_length > 16 {
            println!("Invalid padding length: {}", padding_length);
            return data.to_vec();
        }

        let padded_length = data_length - padding_length;
        if data[padded_length..].iter().all(|&x| x == padding_length as u8) {
            data[..padded_length].to_vec()
        } else {
            data.to_vec()
        }
    }

	fn assemble_gateway_url(&self, path: &str, test_tx: bool) -> String {
        let mut buffer = String::new();
        if test_tx {
            buffer.push_str(if !self.test_gateway_host.is_empty() {
                &self.test_gateway_host
            } else {
                DEFAULT_TEST_GATEWAY_HOST
            });
        } else {
            buffer.push_str(if !self.gateway_host.is_empty() {
                &self.gateway_host
            } else {
                DEFAULT_GATEWAY_HOST
            });
        }
        buffer.push_str(path);
        buffer
    }

	fn assemble_dashboard_url(&self, path: &str) -> String {
        let uri = if self.dashboard_host.is_empty() {
            DEFAULT_DASHBOARD_HOST
        } else {
            &self.dashboard_host
        };
        format!("{}{}", uri, path)
    }

	fn assemble_terminal_url(&self, route: &TerminalRoute, path: &str) -> String {
        let mut buffer = String::new();

        if self.https && route.https {
            buffer.push_str("https://");
        } else {
            buffer.push_str("http://");
        }

        buffer.push_str(&route.ip_address);

        if self.https && route.https {
            buffer.push_str(":8443");
        } else {
            buffer.push_str(":8080");
        }

        buffer.push_str(path);
        buffer
    }

	fn get_timeout(&self, request_timeout: Option<i32>, default: Duration) -> Duration {
        match request_timeout {
            Some(timeout) if timeout > 0 => Duration::from_secs(timeout as u64),
            _ => default,
        }
	}

	fn populate_signature_options<T: DeserializeOwned + Serialize + Clone>(&self, request: &mut T) -> Result<(), Box<dyn std::error::Error>> {
		let sig_opts_result = to_signature_request(request);

        let mut sig_opts = match sig_opts_result {
            Ok(sig_opts) => sig_opts,
            Err(_) => return Ok(()),
        };
        let sig_file = &sig_opts.sig_file;

		if sig_opts.sig_format == SignatureFormat::None {
			if let Some(extension) = Path::new(sig_file).extension().and_then(|s| s.to_str()) {
				sig_opts.sig_format = match extension.to_lowercase().as_str() {
					"png" => SignatureFormat::PNG,
					"jpg" => SignatureFormat::JPG,
					"jpeg" => SignatureFormat::JPG,
					"gif" => SignatureFormat::GIF,
					_ => SignatureFormat::None,
				};
			}
		}

		match sig_opts.sig_format {
			SignatureFormat::None | SignatureFormat::PNG | SignatureFormat::JPG | SignatureFormat::GIF => {
				let modified_request_option = copy_from_signature_request(&sig_opts, request)?;
                if let Some(modified_request) = modified_request_option {
                    *request = modified_request;
                } else {
                    return Err(Box::new(InvalidSignatureFormatError))
                }
				Ok(())
			}
		}
	}

	fn handle_signature<T: Serialize, R: DeserializeOwned + Serialize + Clone>(&self, request: &T, response: &mut R) -> Result<(), Box<dyn std::error::Error>> {
        let request_opts_result = to_signature_request(request);
        let request_opts = match request_opts_result {
            Ok(request_opts) => request_opts,
            Err(_) => return Ok(()),
        };

        let response_opts_result = to_signature_response(response);
        let response_opts = match response_opts_result {
            Ok(response_opts) => response_opts,
            Err(_) => return Ok(()),
        };

		if response_opts.sig_file.is_empty() {
			return Ok(());
		}

        let modified_response = clear_field(response.clone(),"sig_file")?;
        *response = modified_response;

        if !request_opts.sig_file.is_empty() {
            let content = hex::decode(&request_opts.sig_file)?;
            std::fs::write(&request_opts.sig_file, content)?;
        }

		Ok(())
	}

    fn add_api_request_headers(&self, req: RequestBuilder, creds: &APICredentials) -> Result<Request, Box<dyn Error>> {
        let mut headers = APIRequestHeaders {
            api_key: creds.api_key.clone(),
            bearer_token: creds.bearer_token.clone(),
            nonce: self.generate_nonce()?,
			timestamp: Utc::now().to_rfc3339(),
            signature: String::new(), // Placeholder, it will be updated below
        };

        let signature = self.compute_hmac(&headers, &creds.signing_key)?;
        headers.signature = signature;

        let authorization_header = format!("Dual {}:{}:{}", headers.bearer_token, headers.api_key, headers.signature);
        let mut headers_map = HeaderMap::new();

        headers_map.insert("Nonce", HeaderValue::from_str(&headers.nonce)?);
        headers_map.insert("Timestamp", HeaderValue::from_str(&headers.timestamp)?);
        headers_map.insert("Authorization", HeaderValue::from_str(&authorization_header)?);
        headers_map.insert(reqwest::header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers_map.insert(AGENT_HEADER, HeaderValue::from_static(USER_AGENT_STR));

        let request = req.headers(headers_map).build()?;
        Ok(request)
    }

    fn generate_nonce(&self) -> Result<String, Box<dyn Error>> {
        let mut result = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut result);
        Ok(hex::encode(result))
    }

    fn compute_hmac(&self, headers: &APIRequestHeaders, signing_key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let key = hex::decode(signing_key)?;

        let mut buf = Vec::new();
        buf.extend_from_slice(headers.api_key.as_bytes());
        buf.extend_from_slice(headers.bearer_token.as_bytes());
        buf.extend_from_slice(headers.timestamp.as_bytes());
        buf.extend_from_slice(headers.nonce.as_bytes());

        type HmacSha256 = Hmac<Sha256>;        
        let mut mac = <HmacSha256 as hmac::Mac>::new_from_slice(key.as_slice())?;
        mac.update(&buf);

        let result = mac.finalize().into_bytes();
        Ok(hex::encode(result))
    }
}
