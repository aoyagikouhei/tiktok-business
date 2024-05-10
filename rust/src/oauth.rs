use crate::error::Error;
use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use itertools::Itertools;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use rand::Rng;
use reqwest::{header::CACHE_CONTROL, StatusCode};

const AUTH_URL: &str = "https://www.tiktok.com/v2/auth/authorize/";
const TOKEN_URL: &str = "https://business-api.tiktok.com/open_api/v1.3/tt_user/oauth2/token/";
const REFRESH_TOKEN_URL: &str = "https://business-api.tiktok.com/open_api/v1.3/tt_user/oauth2/refresh_token/";
const REVOKE_URL: &str = "https://business-api.tiktok.com/open_api/v1.3/tt_user/oauth2/revoke/";

pub mod scope;
pub use scope::TiktokScope;
use serde::de::DeserializeOwned;
use serde_json::json;

use self::response::{RevokeResponse, TokenResponse};
pub mod response;


#[derive(Debug, Clone)]
pub struct OAuthUrlResult {
    pub oauth_url: String,
    pub csrf_token: String,
}

pub struct TiktokOauth {
    scopes: Vec<TiktokScope>,
    client_key: String,
    client_secret: String,
    callback_url: String,
}

impl TiktokOauth {
    pub fn new(
        client_key: &str,
        client_secret: &str,
        callback_url: &str,
        scopes: Vec<TiktokScope>,
    ) -> Result<Self, Error> {
        Ok(Self {
            callback_url: callback_url.to_owned(),
            scopes,
            client_key: client_key.to_owned(),
            client_secret: client_secret.to_owned(),
        })
    }

    pub fn oauth_url(&self, state: Option<String>) -> OAuthUrlResult {
        let csrf_token = state.unwrap_or(csrf_token());
        let scope = self.scopes.iter().map(|it| it.to_string()).join(",");
        let redirect_uri = utf8_percent_encode(&self.callback_url, NON_ALPHANUMERIC);
        let oauth_url = format!(
            "{}?client_key={}&response_type=code&scope={}&redirect_uri={}&state={}",
            AUTH_URL, self.client_key, scope, redirect_uri, csrf_token
        );
        OAuthUrlResult {
            oauth_url,
            csrf_token,
        }
    }

    pub async fn token(&self, code: &str) -> Result<(TokenResponse, StatusCode), Error> {
        let json = json!({
            "client_id": self.client_key,
            "client_secret": self.client_secret,
            "grant_type": "authorization_code",
            "auth_code": code,
            "redirect_uri": self.callback_url
        });
        make_response(TOKEN_URL, &json).await
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<(TokenResponse, StatusCode), Error> {
        let json = json!({
            "client_id": self.client_key,
            "client_secret": self.client_secret,
            "grant_type": "refresh_token",
            "refresh_token": refresh_token,
        });
        make_response(REFRESH_TOKEN_URL, &json).await
    }

    pub async fn revoke(&self, access_token: &str) -> Result<(RevokeResponse, StatusCode), Error> {
        let json = json!({
            "client_id": self.client_key,
            "client_secret": self.client_secret,
            "access_token": access_token,
        });
        make_response(REVOKE_URL, &json).await
    }
}

async fn execute_send(
    url: &str,
    json: &serde_json::Value,
) -> Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::new()
        .post(url)
        .header(CACHE_CONTROL, "no-cache")
        .json(json)
        .send()
        .await
}

fn csrf_token() -> String {
    let random_bytes: Vec<u8> = (0..16).map(|_| rand::thread_rng().gen::<u8>()).collect();
    println!("{}", random_bytes.len());
    BASE64_URL_SAFE_NO_PAD.encode(random_bytes)
}

async fn make_response<T>(url: &str, json: &serde_json::Value) -> Result<(T, StatusCode), Error> 
where
    T: DeserializeOwned
{
    let response = execute_send(url, json).await?;
    let status_code = response.status();
    let res: T = response.json().await?;
    Ok((res, status_code))
}