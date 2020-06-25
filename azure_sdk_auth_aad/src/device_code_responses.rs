use azure_sdk_core::errors::AzureError;
use oauth2::AccessToken;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DeviceCodeErrorResponse {
    pub error: String,
    pub error_description: String,
    pub error_uri: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCodeAuthorization {
    pub token_type: String,
    pub scope: String,
    pub expires_in: u64,
    pub access_token: AccessToken,
    pub refresh_token: Option<AccessToken>,
    pub id_token: Option<AccessToken>,
}

#[derive(Debug)]
pub enum DeviceCodeError {
    AuthorizationDeclined(DeviceCodeErrorResponse),
    BadVerificationCode(DeviceCodeErrorResponse),
    ExpiredToken(DeviceCodeErrorResponse),
    UnrecognizedError(DeviceCodeErrorResponse),
    UnhandledError(String, String),
    ReqwestError(reqwest::Error),
}

#[derive(Debug, Clone)]
pub enum DeviceCodeResponse {
    AuthorizationSucceded(DeviceCodeAuthorization),
    AuthorizationPending(DeviceCodeErrorResponse),
}

impl TryInto<DeviceCodeResponse> for String {
    type Error = DeviceCodeError;

    fn try_into(self) -> Result<DeviceCodeResponse, Self::Error> {
        // first we try to deserialize as DeviceCodeAuthorization (success)
        match serde_json::from_str::<DeviceCodeAuthorization>(&self) {
            Ok(device_code_authorization) => Ok(DeviceCodeResponse::AuthorizationSucceded(
                device_code_authorization,
            )),
            Err(_) => {
                // now we try to map it to a DeviceCodeErrorResponse
                match serde_json::from_str::<DeviceCodeErrorResponse>(&self) {
                    Ok(device_code_error_response) => {
                        match &device_code_error_response.error as &str {
                            "authorization_pending" => {
                                Ok(DeviceCodeResponse::AuthorizationPending(
                                    device_code_error_response,
                                ))
                            }
                            "authorization_declined" => Err(
                                DeviceCodeError::AuthorizationDeclined(device_code_error_response),
                            ),

                            "bad_verification_code" => Err(DeviceCodeError::BadVerificationCode(
                                device_code_error_response,
                            )),
                            "expired_token" => {
                                Err(DeviceCodeError::ExpiredToken(device_code_error_response))
                            }
                            _ => Err(DeviceCodeError::UnrecognizedError(
                                device_code_error_response,
                            )),
                        }
                    }
                    // If we cannot, we bail out giving the full error as string
                    Err(error) => Err(DeviceCodeError::UnhandledError(error.to_string(), self)),
                }
            }
        }
    }
}
