use futures_util::StreamExt;
use http_body_util::BodyExt;
use hyper::{body::Incoming, Request};

use crate::commons::HandlerError;

pub async fn get_request_body_as_string(request: Incoming) -> Result<String, HandlerError> {
    let collected_request = request.collect().await?.to_bytes().to_vec();
    let parsed_request = String::from_utf8(collected_request)?;
    Ok(parsed_request)
}
