use std::time::Duration;

use crate::{error::Error, URL_PREFIX};
use chrono::prelude::*;
use reqwest::{header::HeaderMap, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;

pub mod get_business_comment_list;
pub mod get_business_get;
pub mod get_business_video_list;
pub mod post_business_comment_reply_create;

const ENV_KEY: &str = "TICTOK_BUSINESS_PREFIX_API";

pub fn clear_prefix_url() {
    std::env::set_var(ENV_KEY, URL_PREFIX);
}

pub fn setup_prefix_url(url: &str) {
    std::env::set_var(ENV_KEY, url);
}

pub(crate) fn make_url(postfix_url: &str, options: &Option<ApiOptions>) -> String {
    make_url_with_prefix(
        &std::env::var(ENV_KEY).unwrap_or(URL_PREFIX.to_owned()),
        options,
        postfix_url,
    )
}

fn make_url_with_prefix(
    default_perfix_url: &str,
    options: &Option<ApiOptions>,
    postfix_url: &str,
) -> String {
    let prefix_url = if let Some(options) = options {
        if let Some(prefix_url) = options.prefix_url.as_ref() {
            prefix_url
        } else {
            default_perfix_url
        }
    } else {
        default_perfix_url
    };
    format!("{}{}", prefix_url, postfix_url)
}

#[derive(Debug, Clone, Default)]
pub struct ApiOptions {
    pub prefix_url: Option<String>,
    pub timeout: Option<Duration>,
}

#[derive(Debug)]
pub struct ApiResponse<T> {
    pub body: T,
    pub status_code: StatusCode,
    pub header: Option<ResponseHeader>,
}

#[derive(Debug)]
pub struct ResponseHeader {
    pub date: DateTime<Utc>,
    pub x_tt_log_id: String,
}

pub async fn execute_api<T>(builder: RequestBuilder) -> Result<ApiResponse<T>, Error>
where
    T: DeserializeOwned,
{
    let response = builder.send().await?;
    let status_code = response.status();
    let header = make_response_header(response.headers());
    let text = match response.text().await {
        Ok(text) => text,
        Err(err) => return Err(Error::Other(format!("{:?}", err), status_code)),
    };
    let json = match serde_json::from_str::<T>(&text) {
        Ok(json) => json,
        Err(_err) => return Err(Error::Other(text, status_code)),
    };
    Ok(ApiResponse {
        body: json,
        status_code,
        header,
    })
}

fn make_response_header(header: &HeaderMap) -> Option<ResponseHeader> {
    let date = header.get("date")?;
    let Ok(date) = date.to_str() else {
        return None;
    };
    let Ok(date) = DateTime::parse_from_rfc2822(date) else {
        return None;
    };
    let x_tt_log_id = header.get("x-tt-logid")?;
    let Ok(x_tt_log_id) = x_tt_log_id.to_str() else {
        return None;
    };
    Some(ResponseHeader {
        date: date.to_utc(),
        x_tt_log_id: x_tt_log_id.to_owned(),
    })
}

pub(crate) fn apply_options(
    client: RequestBuilder,
    options: &Option<ApiOptions>,
) -> RequestBuilder {
    let Some(options) = options else {
        return client;
    };
    let Some(timeout) = options.timeout else {
        return client;
    };
    client.timeout(timeout)
}
