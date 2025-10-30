use crate::responses::comment::Comment;
use crate::{
    apis::{ApiResponse, execute_api},
    error::Error as ApiError,
    options::{TiktokOptions, apply_timeout, make_url},
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/business/comment/list/";

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<TiktokOptions>,
    business_id: String,
    video_id: String,
    comment_ids: Option<Vec<String>>,
    include_replies: Option<bool>,
    status: Option<Status>,
    sort_field: Option<SortField>,
    sort_order: Option<SortOrder>,
    cursor: Option<usize>,
    max_count: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "PUBLIC")]
    Public,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "ALL"),
            Self::Public => write!(f, "PUBLIC"),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::All
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SortField {
    #[serde(rename = "likes")]
    Likes,
    #[serde(rename = "replies")]
    Replies,
    #[serde(rename = "create_time")]
    CreateTime,
}

impl std::fmt::Display for SortField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Likes => write!(f, "likes"),
            Self::Replies => write!(f, "replies"),
            Self::CreateTime => write!(f, "create_time"),
        }
    }
}

impl Default for SortField {
    fn default() -> Self {
        Self::Likes
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
    #[serde(rename = "smart")]
    Smart,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
            Self::Smart => write!(f, "smart"),
        }
    }
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Asc
    }
}

impl Api {
    pub fn new(business_id: &str, video_id: &str, options: Option<TiktokOptions>) -> Self {
        Self {
            options,
            business_id: business_id.to_owned(),
            video_id: video_id.to_owned(),
            ..Default::default()
        }
    }

    pub fn comment_ids(mut self, value: Vec<String>) -> Self {
        self.comment_ids = Some(value);
        self
    }

    pub fn include_replies(mut self, value: bool) -> Self {
        self.include_replies = Some(value);
        self
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }

    pub fn sort_field(mut self, value: SortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn sort_order(mut self, value: SortOrder) -> Self {
        self.sort_order = Some(value);
        self
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
        query_parameters.push(("video_id", self.video_id));
        if let Some(comment_ids) = self.comment_ids {
            query_parameters.push((
                "comment_ids",
                format!("[\"{}\"]", comment_ids.join("\",\"")),
            ));
        }
        if let Some(include_replies) = self.include_replies {
            query_parameters.push(("include_replies", include_replies.to_string()));
        }
        if let Some(status) = self.status {
            query_parameters.push(("status", status.to_string()));
        }
        if let Some(sort_field) = self.sort_field {
            query_parameters.push(("sort_field", sort_field.to_string()));
        }
        if let Some(sort_order) = self.sort_order {
            query_parameters.push(("sort_order", sort_order.to_string()));
        }
        if let Some(cursor) = self.cursor {
            query_parameters.push(("cursor", cursor.to_string()));
        }
        if let Some(max_count) = self.max_count {
            query_parameters.push(("max_count", max_count.to_string()));
        }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<Comment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Data {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .comments
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true);
        if !res {
            println!("Data {:?}", self.extra);
        }
        res
    }
}
