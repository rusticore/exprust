use std::collections::HashMap;

use crate::utils::http;

#[derive(Debug, Default)]
pub struct Request {
  pub method: http::HttpMethod,
  pub uri: String,
  pub version: String,
  pub headers: HashMap<String, String>,
  pub body: Option<String>,
}

impl Request {}
