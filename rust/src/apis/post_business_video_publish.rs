use crate::{
    apis::{ApiResponse, execute_api},
    error::Error as ApiError,
    options::{TiktokOptions, apply_timeout, make_url},
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/business/video/publish/";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PostInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    pub is_brand_organic: bool,
    pub is_branded_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tto_invite_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_comment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_duet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stitch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ai_generated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_to_draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ads_only: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    pub business_id: String,
    pub video_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_thumbnail_url: Option<String>,
    pub post_info: PostInfo,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<TiktokOptions>,
    body: Body,
}

impl Api {
    pub fn new(body: Body, options: Option<TiktokOptions>) -> Self {
        Self { options, body }
    }

    #[allow(clippy::vec_init_then_push)]
    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let client = reqwest::Client::new()
            .post(make_url(URL, &self.options))
            .json(&self.body)
            .header("Access-Token", bearer_code);
        apply_timeout(client, &self.options)
    }

    pub async fn execute(self, bearer_code: &str) -> Result<ApiResponse<Response>, ApiError> {
        execute_api(|| self.clone().build(bearer_code), &self.options).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub request_id: String,
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    pub share_id: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Data {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Data {:?}", self.extra);
        }
        res
    }
}
