use crate::responses::comment::Comment;
use crate::{
    apis::{execute_api, make_url, ApiResponse},
    error::Error as ApiError,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/business/comment/reply/create/";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    pub business_id: String,
    pub video_id: String,
    pub comment_id: String,
    pub text: String,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    body: Body,
}

impl Api {
    pub fn new(body: Body) -> Self {
        Self { body }
    }

    #[allow(clippy::vec_init_then_push)]
    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let client = reqwest::Client::new();
        client
            .post(make_url(URL))
            .json(&self.body)
            .header("Access-Token", bearer_code)
    }

    pub async fn execute(self, bearer_code: &str) -> Result<ApiResponse<Response>, ApiError> {
        execute_api(self.build(bearer_code)).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Comment>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .data
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
