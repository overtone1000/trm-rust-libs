use futures_util::StreamExt;
use http_body_util::BodyExt;
use hyper::{body::Incoming, client::conn::http1::Parts, Request, Response};

use crate::{
    commons::{HandlerError, ResponseGate},
    generic_json_error::generic_json_error_from_debug,
};

pub async fn get_request_body_as_string(request: Incoming) -> Result<String, HandlerError> {
    let collected_request = request.collect().await?.to_bytes().to_vec();
    let parsed_request = String::from_utf8(collected_request)?;
    Ok(parsed_request)
}

pub async fn check_basic_authentication(
    request_parts: hyper::http::request::Parts,
    realm: &str,
    validator: impl Fn(&str) -> bool,
) -> ResponseGate<()> {
    match request_parts
        .headers
        .get(hyper::http::header::AUTHORIZATION)
    {
        Some(auth) => {
            match auth.to_str() {
                Ok(str) => {
                    let words: Vec<&str> = str.split_whitespace().collect();
                    if words[0] == "Basic" && validator(words[1]) {
                        return ResponseGate::Continue();
                    } else {
                        return ResponseGate::ImmediateReturn(
                            Response::builder()
                                .status(hyper::StatusCode::UNAUTHORIZED)
                                .body(())
                                .expect("Response should build."),
                        );
                    }
                }
                Err(e) => {
                    return ResponseGate::Error(Box::new(e));
                }
            };
        }
        None => ResponseGate::ImmediateReturn(
            Response::builder()
                .status(hyper::StatusCode::UNAUTHORIZED)
                .header(
                    hyper::header::WWW_AUTHENTICATE,
                    "Basic realm=\"".to_string() + realm + "\"",
                )
                .body(())
                .expect("Response should build."),
        ),
    }
}
