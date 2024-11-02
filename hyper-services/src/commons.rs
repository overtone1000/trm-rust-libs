use std::{future::Future, pin::Pin};

use futures_util::{future::BoxFuture, TryStreamExt};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
    body::{Body, Bytes, Frame, Incoming},
    header::HeaderValue,
    Request, Response,
};
use serde::Serialize;

pub type HandlerBody = BoxBody<Bytes, HandlerError>;
pub type HandlerError = Box<dyn std::error::Error + Send + Sync>;
pub type HandlerResult = Result<Response<HandlerBody>, HandlerError>;
pub type HandlerFuture = Pin<Box<dyn Future<Output = HandlerResult> + Send>>;

pub enum Handler {
    Continue,
    ImmediateReturn(Response<HandlerBody>),
    Error(HandlerError),
}
