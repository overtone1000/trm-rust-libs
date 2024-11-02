use std::marker::PhantomData;

use hyper::{body::Incoming, service::Service, Request, Response};

use crate::commons::{HandlerBody, HandlerError, HandlerFuture, HandlerResult};

#[trait_variant::make(StatelessHandler: Send)]
pub trait LocalStatelessHandler: Clone {
    async fn handle_request(request: Request<Incoming>) -> HandlerResult;
}

#[derive(Clone)]
pub struct StatelessService<T>
where
    T: StatelessHandler,
{
    phantom_handler: PhantomData<T>,
}

impl<T> StatelessService<T>
where
    T: StatelessHandler,
{
    pub fn create() -> StatelessService<T> {
        StatelessService {
            phantom_handler: PhantomData,
        }
    }
}

impl<T> Service<Request<Incoming>> for StatelessService<T>
where
    T: StatelessHandler + 'static,
{
    type Response = Response<HandlerBody>;
    type Error = HandlerError;
    type Future = HandlerFuture;

    fn call(&self, request: Request<Incoming>) -> Self::Future {
        Box::pin(T::handle_request(request))
    }
}

//expected `impl Future<Output = Result<Response<BoxBody<Bytes, Box<dyn Error + Send + Sync>>>, Box<dyn Error + Send + Sync>>>`
// to be a future that resolves to
//`dyn Future<Output = Result<Response<BoxBody<Bytes, Box<dyn Error + Send + Sync>>>, Box<dyn Error + Send + Sync>>> + Send`
//, but it resolves to `Result<Response<BoxBody<Bytes, Box<dyn Error + Send + Sync>>>, Box<dyn Error + Send + Sync>>`
//expected trait object `dyn Future<Output = Result<Response<BoxBody<Bytes, Box<dyn Error + Send + Sync>>>, Box<...>>> + Send`
