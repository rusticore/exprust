use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum HttpMethod {
  #[default]
  GET,
  POST,
  PUT,
  PATCH,
  DELETE,
  HEAD,
  OPTIONS,
  CONNECT,
  TRACE,
  OTHER(String),
}

#[derive(Debug, Default)]
pub enum HttpStatusCode {
  #[default]
  Success = 200,
  BadRequest = 400,
  Unauthorized = 401,
  Forbidden = 403,
  NotFound = 404,
  MethodNotAllowed = 405,
  NotAcceptable = 406,
  Conflict = 409,
  InternalServerError = 500,
  NotImplemented = 501,
  BadGateway = 502,
  ServiceUnavailable = 503,
}
