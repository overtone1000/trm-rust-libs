use hyper::{body::Incoming, service::Service, Request};

use crate::commons::{HandlerError, HandlerFuture, HandlerResponse, HandlerResult};

#[trait_variant::make(StatefulHandler: Send)]
pub trait LocalStatefulHandler: Clone {
    async fn handle_request(self: Self, request: Request<Incoming>) -> HandlerResult;
}

#[derive(Clone)]
pub struct StatefulService<T>
where
    T: StatefulHandler,
{
    handler: T,
}

impl<T> StatefulService<T>
where
    T: StatefulHandler,
{
    pub fn create(handler: T) -> StatefulService<T> {
        StatefulService { handler: handler }
    }
}

impl<T> Service<Request<Incoming>> for StatefulService<T>
where
    T: StatefulHandler + 'static,
{
    type Response = HandlerResponse;
    type Error = HandlerError;
    type Future = HandlerFuture;

    fn call(&self, request: Request<Incoming>) -> Self::Future {
        Box::pin(T::handle_request(self.handler.clone(), request))
    }
}

//expected `impl Future<Output = Result<Response<BoxBody<Bytes, Box<dyn Error + Send + Sync>>>, Box<dyn Error + Send + Sync>>>`
// to be a future that resolves to
//`dyn Future<Output = Result<Response<BoxBody<Bytes, Box<dyn Error + Send + Sync>>>, Box<dyn Error + Send + Sync>>> + Send`
//, but it resolves to `Result<Response<BoxBody<Bytes, Box<dyn Error + Send + Sync>>>, Box<dyn Error + Send + Sync>>`
//expected trait object `dyn Future<Output = Result<Response<BoxBody<Bytes, Box<dyn Error + Send + Sync>>>, Box<...>>> + Send`
