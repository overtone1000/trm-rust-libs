use rustls_pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};

pub struct TlsCerts
{
    pub certs:Vec<CertificateDer<'static>>,
    pub keys:PrivateKeyDer<'static>,
}

pub fn generate_simple_certificates<S:Into<Vec<String>>>(hostnames:S)->Result<TlsCerts,Box<rcgen::Error>>
{
    match rcgen::generate_simple_self_signed(hostnames)
    {
        Ok(keypair)=>{
            
            let certs =  vec![CertificateDer::from(keypair.cert)];
            let keys:PrivateKeyDer<'_> = PrivatePkcs8KeyDer::from(keypair.signing_key.serialize_der()).into();
            
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