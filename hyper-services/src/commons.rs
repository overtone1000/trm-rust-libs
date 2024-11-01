use std::{future::Future, pin::Pin};

use futures_util::{future::BoxFuture, TryStreamExt};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
    body::{Bytes, Frame, Incoming},
    header::HeaderValue,
    Request, Response,
};

pub enum HandlerBody {
    Empty,
    BoxBody(BoxBody<Bytes, HandlerError>),
}

pub type HandlerError = Box<dyn std::error::Error + Send + Sync>;
//pub type HandlerBody = BoxBody<Bytes, HandlerError>;
pub type HandlerResponse<T> = Response<T>;
pub type HandlerResult<T> = Result<HandlerResponse<T>, HandlerError>;
pub type HandlerFuture<T> = Pin<Box<dyn Future<Output = HandlerResult<T>> + Send>>;

pub enum ResponseGate<T> {
    Continue(),
    ImmediateReturn(HandlerResponse<T>),
    Error(HandlerError),
}
