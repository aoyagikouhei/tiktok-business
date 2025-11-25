use crate::{
    apis::{ApiResponse, execute_api},
    error::Error as ApiError,
    options::{TiktokOptions, apply_timeout, make_url},
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/business/publish/status/";

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<TiktokOptions>,
    business_id: String,
    publish_id: String,
}

impl Api {
    pub fn new(business_id: &str, publish_id: &str, options: Option<TiktokOptions>) -> Self {
        Self {
            options,
            business_id: business_id.to_owned(),
            publish_id: publish_id.to_owned(),
        }
    }

    #[allow(clippy::vec_init_then_push)]
    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("business_id", self.business_id));
        query_parameters.push(("publish_id", self.publish_id));
        let client = reqwest::Client::new()
            .get(make_url(URL, &self.options))
            .query(&query_parameters)
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
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
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
