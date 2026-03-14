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

pub struct ConnectionProperties
{
    pub with_upgrades:bool
}

impl Default for ConnectionProperties
{
    fn default() -> Self {
        Self { 
            with_upgrades: false
        }
    }
}

pub(crate) async fn spawn_server<S>(
    ip: IpAddr,
    port: u16,
    service: S,
    props: ConnectionProperties
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

    println!("Binding to {}:{}", ip, port);

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

    println!("Starting listen loop on {}:{}", ip, port);
    loop {
        match listener.accept().await {
            Ok((tcp, _)) => {
                //Need to spawn these as separate tasks...
                let io = TokioIo::new(tcp);
                let clone = service.clone();

                let connection: http1::Connection<TokioIo<tokio::net::TcpStream>, S>=http1::Builder::new()
                        .timer(TokioTimer::new())
                        .serve_connection(io, clone);


                match props.with_upgrades
                {
                    true=>tokio::task::spawn(async move {handle_result(connection.with_upgrades().await)}),
                    false=>tokio::task::spawn(async move {handle_result(connection.await)})
                };

                //Can't await, must be handled in spawned task!
                //match result.await
                //{
                //    Ok(_)=>(),
                //    Err(e)=>println!("Error serving connection: {:?}", e)
                //};
            }
            Err(_) => {
                eprintln!("Couldn't accept tcp, retrying.")
            }
        };
    }
}

fn handle_result<T:std::error::Error>(result:Result<(),T>)->()
{
    match result
    {
        Ok(_)=>(),
        Err(e)=>eprintln!("Listener error {:?}. Could this be a misconfiguration of the service spawner in trm-rust-libs?",e)
    }
}