use std::net::IpAddr;

use hyper::{body::Incoming, service::Service, Request, Response};

use crate::{commons::{HandlerBody, HandlerError, HandlerFuture, HandlerResult}, service::spawn::{ConnectionProperties, spawn_server}};

#[trait_variant::make(StatefulHandler: Send)]
pub trait _LocalStatefulHandler: Clone {
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
    T: StatefulHandler+'static,
{
    pub fn create(handler: T) -> StatefulService<T> {
        StatefulService { handler: handler }
    }

    pub async fn start(
        self,
        ip: IpAddr,
        port: u16,
        //service: StatefulService<T>,
        props: ConnectionProperties
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    {
        spawn_server(
            ip,
            port,
            self,
            props
        ).await
    }

    pub fn get_handler(&mut self)->&mut T
    {
        &mut self.handler
    }
}

impl<T> Service<Request<Incoming>> for StatefulService<T>
where
    T: StatefulHandler + 'static,
{
    type Response = Response<HandlerBody>;
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
