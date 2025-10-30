use crate::error::Error;
use chrono::prelude::*;
use reqwest::{RequestBuilder, StatusCode, header::HeaderMap};
use serde::de::DeserializeOwned;

pub mod get_business_comment_list;
pub mod get_business_get;
pub mod get_business_video_list;
pub mod post_business_comment_reply_create;

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
