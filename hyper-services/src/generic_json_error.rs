use hyper::Response;
use serde::Serialize;

use crate::response_building::full_to_boxed_body;

#[derive(Serialize)]
struct ErrorContents {
    message: String,
}

#[derive(Serialize)]
struct Error {
    error: ErrorContents,
}

pub fn generic_json_error(
    message: &str,
) -> Response<
    http_body_util::combinators::BoxBody<
        hyper::body::Bytes,
        Box<dyn std::error::Error + Send + Sync>,
    >,
> {
    let e = Error {
        error: ErrorContents {
            message: message.to_string(),
        },
    };

    Response::new(full_to_boxed_body(
        serde_json::to_string(&e).expect("Couldn't serialize error message."),
    ))
}

pub fn generic_json_error_from_debug<T>(
    e: T,
) -> Response<
    http_body_util::combinators::BoxBody<
        hyper::body::Bytes,
        Box<dyn std::error::Error + Send + Sync>,
    >,
>
where
    T: std::fmt::Debug,
{
    generic_json_error(&format!("Error: {:?}", e))
}
