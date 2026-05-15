use crate::platform::HASMQTTState;

impl HASMQTTState for String
{
    fn to_payload(&self)->&str {
        self
    }
    
    fn from_payload(payload:bytes::Bytes)->Result<String, Box<dyn std::error::Error + 'static>> {
        match std::str::from_utf8(&payload)
        {
            Ok(str)=>{
                Ok(str.to_owned())
            }
            Err(e)=>{
                eprintln!("Couldn't convert payload {:?} to string.",payload);
                Err(Box::new(e))
            }
        }
    }
}