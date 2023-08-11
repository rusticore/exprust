use crate::http::HttpMethod;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Request {
  pub method: HttpMethod,
  pub uri: String,
  pub version: String,
  pub headers: HashMap<String, String>,
  pub body: Option<String>,
}

impl Request {}
