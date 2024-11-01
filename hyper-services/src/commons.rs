use std::{future::Future, pin::Pin};

use futures_util::{future::BoxFuture, TryStreamExt};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
    body::{Bytes, Frame, Incoming},
    header::HeaderValue,
    Request, Response,
};

pub type HandlerError = Box<dyn std::error::Error + Send + Sync>;
pub type HandlerBody = BoxBody<Bytes, HandlerError>;
pub type HandlerResponse = Response<HandlerBody>;
pub type HandlerResult = Result<HandlerResponse, HandlerError>;
pub type HandlerFuture = Pin<Box<dyn Future<Output = HandlerResult> + Send>>;

pub enum ResponseProcessingStepResult {
    Continue(HandlerResponse),
    Complete(HandlerResponse),
    Error(HandlerError),
}
