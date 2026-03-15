use base64::Engine;
use http_body_util::BodyExt;
use hyper::{body::Incoming, Response};

use crate::{
    commons::{Handler, HandlerError}, response_building::empty_body,
};

pub async fn get_request_body_as_string(request: Incoming) -> Result<String, HandlerError> {
    let collected_request = request.collect().await?.to_bytes().to_vec();
    let parsed_request = String::from_utf8(collected_request)?;
    Ok(parsed_request)
}

const DECODER: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
    &base64::alphabet::STANDARD,
    base64::engine::GeneralPurposeConfig::new(),
);


#[derive(Debug,Clone,PartialEq)]
pub struct Auth {
    pub user:String,
    pub password:String
}

fn basic_authentication_decode(encoded: &str) -> Option<Auth> {
    match DECODER.decode(encoded) {
        Ok(result) => match String::from_utf8(result) {
            Ok(result) => {
                let split: Vec<&str> = result.split_terminator(':').collect();
                match split.len() {
                    2 => Some(Auth{user:split[0].to_string(), password:split[1].to_string()}),
                    _ => {
                        eprintln!("Wrong number of split entries in {}", result);
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
                None
            }
        },
        Err(e) => {
            eprintln!("{:?}", e);
            None
        }
    }
}

fn unauthorized_response(realm: &str) -> Handler {
    Handler::ImmediateReturn(
        Response::builder()
            .status(hyper::StatusCode::UNAUTHORIZED)
            .header(
                hyper::header::WWW_AUTHENTICATE,
                "Basic realm=\"".to_string() + realm + "\"",
            )
            .body(empty_body())
            .expect("Response should build."),
    )
}

pub async fn check_basic_authentication(
    request_parts: &hyper::http::request::Parts,
    realm: &str,
    validator: impl Fn(Auth) -> bool,
) -> Handler {
    match request_parts
        .headers
        .get(hyper::http::header::AUTHORIZATION)
    {
        Some(auth) => match auth.to_str() {
            Ok(str) => {
                let auth_words: Vec<&str> = str.split_whitespace().collect();
                match auth_words[0] == "Basic" {
                    true => {
                        match basic_authentication_decode(auth_words[1])
                        {
                            Some(auth) => {
                                if validator(auth)
                                {
                                    Handler::Continue
                                }
                                else {
                                    unauthorized_response(realm)                            
                                }
                            },
                            None => unauthorized_response(realm),
                        }
                    },
                    false => unauthorized_response(realm),
                }
            }
            Err(e) => Handler::Error(Box::new(e)),
        },
        None => unauthorized_response(realm),
    }
}
