use std::{future::Future, pin::Pin};

use futures_util::{future::BoxFuture, TryStreamExt};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
    body::{Bytes, Frame, Incoming},
    Request, Response,
};

use tokio_util::io::ReaderStream;

pub type HandlerError = Box<dyn std::error::Error + Send + Sync>;
pub type HandlerBody = BoxBody<Bytes, HandlerError>;
pub type HandlerResponse = Response<HandlerBody>;
pub type HandlerResult = Result<HandlerResponse, HandlerError>;
pub type HandlerFuture = Pin<Box<dyn Future<Output = HandlerResult> + Send>>;
//pub type HandlerFuture = BoxFuture<'static, dyn Future<Output = HandlerResult> + Send>;
//pub type HandlerFuture = dyn Future<Output = Result<HandlerResponse, HandlerError>>;

pub async fn get_request_body_as_string(
    request: Request<Incoming>,
) -> Result<String, HandlerError> {
    Ok(
        String::from_utf8(request.collect().await?.to_bytes().to_vec())
            .expect("Couldn't parse bytes."),
    )
}

pub fn full_to_boxed_body<T: Into<Bytes>>(chunk: T) -> HandlerBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub fn stream_to_boxed_body(stream: ReaderStream<tokio::fs::File>) -> HandlerBody {
    let remapped_stream = stream.map_err(|e| match e {
        e => Box::new(e) as HandlerError,
    });
    let stream_body = http_body_util::StreamBody::new(remapped_stream.map_ok(Frame::data));
    stream_body.boxed()
}

pub fn not_found() -> HandlerResponse {
    Response::builder()
        .status(hyper::StatusCode::NOT_FOUND)
        .body(full_to_boxed_body("Resource not found."))
        .expect("Should produce response.")
}

pub fn bad_request() -> HandlerResponse {
    Response::builder()
        .status(hyper::StatusCode::BAD_REQUEST)
        .body(full_to_boxed_body("Malformed request."))
        .expect("Should produce response.")
}

pub async fn send_file(path: String) -> HandlerResult {
    if path.contains("..") {
        //Reject attempts to access parent directories
        return Ok(bad_request());
    } else {
        let path = ".".to_string() + path.as_str(); //need to prepend to get to this file system.
        eprintln!("Need to point this at a safe directory to avoid inappropriately exposing files in the working directory.");

        println!("Trying to open file {}", path);
        match tokio::fs::File::open(path).await {
            Ok(file) => {
                let reader_stream: tokio_util::io::ReaderStream<tokio::fs::File> =
                    tokio_util::io::ReaderStream::new(file);
                let boxed_body = stream_to_boxed_body(reader_stream);

                // Send response
                let response = Response::builder()
                    .status(hyper::StatusCode::OK)
                    .body(boxed_body)
                    .unwrap();

                Ok(response)
            }
            Err(e) => {
                eprintln!("{:?}", e);
                return Ok(not_found());
            }
        }
    }
}
