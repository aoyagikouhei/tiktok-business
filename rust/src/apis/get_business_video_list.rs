use crate::responses::video::Video;
use crate::responses::video::VideoField;
use crate::{
    apis::{execute_api, make_url, ApiResponse},
    error::Error as ApiError,
};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "/business/video/list/";

#[derive(Debug, Clone, Default)]
pub struct Api {
    business_id: String,
    fields: HashSet<VideoField>,
    cursor: Option<usize>,
    max_count: Option<usize>,
}

impl Api {
    pub fn new(business_id: &str, fields: HashSet<VideoField>) -> Self {
        Self {
            business_id: business_id.to_owned(),
            fields,
            ..Default::default()
        }
    }

    pub fn cursor(mut self, value: usize) -> Self {
        self.cursor = Some(value);
        self
    }

    pub fn max_count(mut self, value: usize) -> Self {
        self.max_count = Some(value);
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("business_id", self.business_id));
        query_parameters.push((
            "fields",
            format!("[\"{}\"]", self.fields.iter().join("\",\"")),
        ));
        if let Some(cursor) = self.cursor {
            query_parameters.push(("cursor", cursor.to_string()));
        }
        if let Some(max_count) = self.max_count {
            query_parameters.push(("max_count", max_count.to_string()));
        }
        let client = reqwest::Client::new();
        client
            .get(make_url(URL))
            .query(&query_parameters)
            .bearer_auth(bearer_code)
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<Video>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Data {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .videos
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true);
        if !res {
            println!("Data {:?}", self.extra);
        }
        res
    }
}
