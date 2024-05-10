use serde::{Deserialize, Serialize};

pub type TokenResponse = Respense<TokenData>;
pub type RevokeResponse = Respense<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Respense<T> {
    pub code: u64,
    pub message: String,
    pub data: Option<T>,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub open_id: String,
    pub scope: String,
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub refresh_token_expires_in: u64,
    pub token_type: String,
}
