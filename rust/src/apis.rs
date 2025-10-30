use std::time::Duration;

use crate::{error::Error, options::TiktokOptions};
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

pub(crate) async fn execute_api<T>(
    f: impl Fn() -> RequestBuilder,
    options: &Option<TiktokOptions>,
) -> Result<ApiResponse<T>, Error>
where
    T: DeserializeOwned,
{
    let res = reqwest_builder_retry::convenience::execute(
        |_| f(),
        |response| {
            reqwest_builder_retry::convenience::json::check_done::<T>(
                response,
                &[
                    StatusCode::TOO_MANY_REQUESTS,
                    StatusCode::INTERNAL_SERVER_ERROR,
                ],
            )
        },
        options.as_ref().and_then(|opt| opt.try_count).unwrap_or(0), // リトライ回数
        options
            .as_ref()
            .and_then(|opt| opt.retry_duration)
            .unwrap_or(Duration::from_secs(2)), // リトライ間隔
    )
    .await;

    match res {
        Ok(response) => {
            let header = make_response_header(&response.headers);
            Ok(ApiResponse {
                body: response.data,
                status_code: response.status_code,
                header,
            })
        }
        Err(err) => match err {
            reqwest_builder_retry::error::Error::NoTry => Err(Error::Invalid("NoTry".into())),
            reqwest_builder_retry::error::Error::Stop(res) => Err(convert_response_error(res)),
            reqwest_builder_retry::error::Error::TryOver(res) => Err(convert_response_error(res)),
        },
    }
}

fn convert_response_error(err: reqwest_builder_retry::convenience::json::ResponseError) -> Error {
    if let Some(response_data) = err.response_data {
        Error::Other(response_data.body, response_data.status_code)
    } else if let Some(error) = err.error {
        Error::Reqwest(error)
    } else {
        Error::Invalid("ResponseError Invalid".into())
    }
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
