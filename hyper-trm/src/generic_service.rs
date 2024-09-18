use http_body_util::combinators::BoxBody;
use hyper::{body::Bytes, Response};

pub type HandlerError = Box<dyn std::error::Error + Send + Sync>;
pub type HandlerBody = BoxBody<Bytes, HandlerError>;
pub type HandlerResponse = Response<HandlerBody>;
pub type HandlerResult = Result<HandlerResponse, HandlerError>;
