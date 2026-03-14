use std::net::{IpAddr, SocketAddr};

use hyper::{
    body::{Body, Incoming},
    server::conn::http1,
    service::HttpService,
};
use hyper_util::rt::{TokioIo, TokioTimer};

use service::stateless_service::{StatelessHandler, StatelessService};
use tokio::net::TcpListener;
use tokio_rustls::{TlsAcceptor, rustls::ServerConfig};

use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};

pub mod commons;
pub mod cors;
pub mod generic_json_error;
pub mod request_processing;
pub mod response_building;
pub mod service;

pub struct TlsCerts
{
    pub certs:Vec<CertificateDer<'static>>,
    pub keys:PrivateKeyDer<'static>
}

pub struct ConnectionProperties
{
    pub with_upgrades:bool,
    pub tls:Option<TlsCerts>
}

impl Default for ConnectionProperties
{
    fn default() -> Self {
        Self { 
            with_upgrades: false,
            tls: None
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

    let tls_handler = match props.tls
    {
        Some(certs)=>{
             // Load public certificate.
                        
            let mut server_config = match ServerConfig::builder()
                .with_no_client_auth()
                .with_single_cert(certs.certs, certs.keys)
                .map_err(|e|  std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
                {
                    Ok(config)=>config,
                    Err(e)=>{
                        eprintln!("Couldn't initialize tls handler");
                        return Err(Box::new(e));
                    }
                };
            server_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];
            let tls_acceptor = TlsAcceptor::from(std::sync::Arc::new(server_config));
            Some(tls_acceptor)
        }
        None=>None
    };

    println!("Starting listen loop on {}:{}", ip, port);
    loop {
        match listener.accept().await {
            Ok((tcp, _)) => {
                
                let clone = service.clone();

                match &tls_handler
                {
                    Some(tls)=>{
                        let tls = tls.clone();

                        tokio::spawn(async move {
                            let tls_stream = match tls.accept(tcp).await {
                                Ok(tls_stream) => tls_stream,
                                Err(err) => {
                                    eprintln!("failed to perform tls handshake: {err:#}");
                                    return;
                                }
                            };

                            service_connection(tls_stream, clone, props.with_upgrades).await
                        });
                    },
                    None=>{
                        service_connection(tcp, clone, props.with_upgrades).await
                    }
                }

                /*
                //Old pattern was to spawn handler. Necessary still?
                match props.with_upgrades
                {
                    true=>tokio::task::spawn(async move {handle_result(connection.with_upgrades().await)}),
                    false=>tokio::task::spawn(async move {handle_result(connection.await)})
                };     
                */
            }
            Err(_) => {
                eprintln!("Couldn't accept tcp, retrying.")
            }
        };
    }
}

async fn service_connection<StreamType,S>(stream:StreamType, service_clone:S, with_upgrades:bool)->()
where
    S: 'static + Clone + Send + HttpService<Incoming>,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    S::ResBody: 'static,
    <S::ResBody as Body>::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    <S as HttpService<hyper::body::Incoming>>::ResBody: Send,
    <S as HttpService<hyper::body::Incoming>>::Future: Send,
    <<S as HttpService<hyper::body::Incoming>>::ResBody as hyper::body::Body>::Data: Send,
    StreamType: 'static + tokio::io::AsyncRead+tokio::io::AsyncWrite+std::marker::Unpin+std::marker::Send
{
    tokio::spawn(async move {
            let io = TokioIo::new(stream);

            let connection=http1::Builder::new()
                .timer(TokioTimer::new())
                .serve_connection(io, service_clone);

            match with_upgrades
            {   
                true=>handle_result(connection.with_upgrades().await),
                false=>handle_result(connection.await)
            };
        }
    );
}

fn handle_result<T:std::error::Error>(result:Result<(),T>)->()
{
    match result
    {
        Ok(_)=>(),
        Err(e)=>eprintln!("Listener error {:?}. Could this be a misconfiguration of the service spawner in trm-rust-libs?",e)
    }
}