use hyper::{body::Incoming, service::Service, Request};

use crate::commons::{HandlerError, HandlerFuture, HandlerResponse, HandlerResult};

pub trait Handler {
    async fn handle_request<'a>(&'a self, request: Request<Incoming>) -> HandlerResult;
}

#[derive(Clone)]
pub struct GenericService<T>
where
    T: Handler,
{
    handler: T,
}

impl<T> GenericService<T> where T: Handler {}

/*
impl<T> Service<Request<Incoming>> for GenericService<T>
where
    T: Handler,
{
    type Response = HandlerResponse;
    type Error = HandlerError;
    type Future = HandlerFuture;

    fn call<'a>(&'a self, request: Request<Incoming>) -> Self::Future {
        let result = self.handler.handle_request(request);
        //let result = self.get_response(request);
        let retval = Box::pin(result);
        retval
    }
}
*/
