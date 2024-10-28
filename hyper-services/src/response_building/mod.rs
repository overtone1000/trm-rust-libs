use std::{fs::FileType, future::Future, pin::Pin};

use futures_util::{future::BoxFuture, TryStreamExt};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
    body::{Bytes, Frame, Incoming},
    header::HeaderValue,
    Request, Response,
};

use tokio_util::io::ReaderStream;

use crate::commons::{HandlerBody, HandlerError, HandlerResponse, HandlerResult};

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

const SUFFIXES_TO_TRY: [&str; 3] = ["", ".html", "/index.html"];
pub async fn send_file(file_system_root_directory: &str, request_path: &str) -> HandlerResult {
    if request_path.contains("..") {
        //Reject attempts to access parent directories
        return Ok(bad_request());
    } else {
        let mut path = file_system_root_directory.to_string() + request_path; //need to prepend to get to this file system.

        if path.ends_with("/") {
            path = path.split_at(path.len() - 1).0.to_string();
        }
        for suffix in SUFFIXES_TO_TRY {
            let final_path = path.to_string() + suffix;
            match tokio::fs::File::open(&final_path).await {
                Ok(file) => match file.metadata().await {
                    Ok(meta) => {
                        if meta.is_file() {
                            let suffix = final_path.split_terminator(".").last();
                            let content_type = match suffix {
                                Some(suffix) => match suffix {
                                    "html" => "text/html",
                                    "js" => "text/javascript",
                                    "ico" => " image/x-icon",
                                    "txt" => "text/plain",
                                    "css" => "text/css",
                                    "csv" => "text/csv",
                                    "jpg" | "jpeg" => "image/jpeg",
                                    "png" => "image/png",
                                    "tif" | "tiff" => "image/tiff",
                                    _ => {
                                        eprintln!(
                                            "Couldn't determine file type for {}",
                                            final_path
                                        );
                                        "text/plain"
                                    }
                                },
                                None => {
                                    eprintln!("Couldn't determine file type for {}", final_path);
                                    "text/plain"
                                }
                            };
                            let reader_stream: tokio_util::io::ReaderStream<tokio::fs::File> =
                                tokio_util::io::ReaderStream::new(file);
                            let boxed_body = stream_to_boxed_body(reader_stream);

                            // Send response
                            let response = Response::builder()
                                .status(hyper::StatusCode::OK)
                                .header("Content-Type", content_type)
                                .body(boxed_body)
                                .unwrap();

                            return Ok(response);
                        }
                    }
                    Err(_) => {}
                },
                Err(_) => {}
            }
        }
        return Ok(not_found());
    }
}
