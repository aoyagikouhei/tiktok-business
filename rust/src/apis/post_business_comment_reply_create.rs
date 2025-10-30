use crate::responses::{create_reply::CreateReply};
use reqwest::RequestBuilder;
use serde::{Serialize, Deserialize};
use crate::{
    options::{make_url, apply_timeout, TiktokOptions},
    apis::{execute_api, ApiResponse},
    error::Error as ApiError,
};

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
    options: Option<TiktokOptions>,
    body: Body,
}





impl Api {
    pub fn new(body: Body, options: Option<TiktokOptions>) -> Self {
        Self {
            options,
            body,
        }
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
        execute_api(self.build(bearer_code)).await
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub request_id: String, 
    pub code: i64, 
    pub message: String, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateReply>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.data.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("Response {:?}", self.extra);
        }
        res
    }
}


