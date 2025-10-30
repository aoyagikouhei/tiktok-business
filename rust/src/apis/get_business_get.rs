use itertools::Itertools;
use std::collections::HashSet;
use crate::responses::{account::Account};
use crate::responses::{account::AccountField};
use reqwest::RequestBuilder;
use serde::{Serialize, Deserialize};
use crate::{
    options::{make_url, apply_timeout, TiktokOptions},
    apis::{execute_api, ApiResponse},
    error::Error as ApiError,
};

const URL: &str = "/business/get/";



#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<TiktokOptions>,
    business_id: String,
    start_date: Option<String>,
    end_date: Option<String>,
    fields: HashSet<AccountField>,
}





impl Api {
    pub fn new(business_id: &str, fields: HashSet<AccountField>, options: Option<TiktokOptions>) -> Self {
        Self {
            options,
            business_id: business_id.to_owned(),
            fields,
            ..Default::default()
        }
    }
    
    pub fn start_date(mut self, value: &str) -> Self {
        self.start_date = Some(value.to_owned());
        self
    }
    
    pub fn end_date(mut self, value: &str) -> Self {
        self.end_date = Some(value.to_owned());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        
        let mut query_parameters = vec![];
        query_parameters.push(("business_id", self.business_id));
        query_parameters.push(("fields", format!("[\"{}\"]", self.fields.iter().join("\",\""))));
        if let Some(start_date) = self.start_date {
            query_parameters.push(("start_date", start_date));
        }
        if let Some(end_date) = self.end_date {
            query_parameters.push(("end_date", end_date));
        }
        let client = reqwest::Client::new()
            .get(make_url(URL, &self.options))
            .query(&query_parameters)
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
    pub data: Option<Account>, 
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


