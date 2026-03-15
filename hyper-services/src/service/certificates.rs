use rustls::pki_types::{CertificateDer, PrivateKeyDer};

pub struct TlsCerts
{
    pub certs:Vec<CertificateDer<'static>>,
    pub keys:PrivateKeyDer<'static>
}

pub fn generate_simple_certificates<S:Into<Vec<String>>>(hostnames:S)->Result<TlsCerts,Box<rcgen::Error>>
{
    match rcgen::generate_simple_self_signed(hostnames)
    {
        Ok(keypair)=>{
            
            let certs =  vec![rustls::pki_types::CertificateDer::from(keypair.cert)];
            let keys = rustls::pki_types::PrivateKeyDer::from(keypair.signing_key);

            Ok(TlsCerts{
                certs,
                keys
            })
        },
        Err(e)=>{
            eprintln!("Couldn't create certificates. {:?}",e);
            Err(Box::new(e))
        }
    }
}