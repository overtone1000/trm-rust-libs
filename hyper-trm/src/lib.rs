use std::net::{IpAddr, SocketAddr};

use hyper::{
    body::{Body, Incoming},
    server::conn::http1,
    service::HttpService,
};
use hyper_util::rt::{TokioIo, TokioTimer};

use tokio::net::TcpListener;

pub mod commons;

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
    let listener = TcpListener::bind(socket).await?;

    loop {
        let (tcp, _) = listener.accept().await?;
        //let (call_tcp_stream, _) = call_listener.accept().await?;

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
}
