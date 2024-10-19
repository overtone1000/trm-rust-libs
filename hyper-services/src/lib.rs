use std::net::{IpAddr, SocketAddr};

use hyper::{
    body::{Body, Incoming},
    server::conn::http1,
    service::HttpService,
};
use hyper_util::rt::{TokioIo, TokioTimer};

use service::stateless_service::{StatelessHandler, StatelessService};
use tokio::net::TcpListener;

pub mod commons;
pub mod cors;
pub mod generic_json_error;
pub mod request_processing;
pub mod response_building;
pub mod service;

pub async fn spawn_server<S>(
    ip: IpAddr,
    port: u16,
    service: S,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    S: 'static + Clone + Send + HttpService<Incoming>,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    S::ResBody: 'static,
    <S::ResBody as Body>::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    <S as HttpService<hyper::body::Incoming>>::ResBody: Send,
    <S as HttpService<hyper::body::Incoming>>::Future: Send,
    <<S as HttpService<hyper::body::Incoming>>::ResBody as hyper::body::Body>::Data: Send,
{
    let socket = SocketAddr::new(ip, port);

    let listener: TcpListener;
    loop {
        match TcpListener::bind(socket).await {
            Ok(res) => {
                listener = res;
                break;
            }
            Err(_) => {
                eprintln!("Couldn't bind port. Retrying.");
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            }
        }
    }

    loop {
        match listener.accept().await {
            Ok((tcp, _)) => {
                //Need to spawn these as separate tasks...
                let io = TokioIo::new(tcp);
                let clone = service.clone();

                tokio::task::spawn(async move {
                    // Handle the connection from the client using HTTP1 and pass any
                    // HTTP requests received on that connection to the `hello` function
                    if let Err(err) = http1::Builder::new()
                        .timer(TokioTimer::new())
                        .serve_connection(io, clone)
                        .await
                    {
                        println!("Error serving connection: {:?}", err);
                    }
                });
            }
            Err(_) => {
                eprintln!("Couldn't accept tcp, retrying.")
            }
        };
    }
}
