use crate::platform::HASMQTTState;

const COMMAND_ON:&str="ON"; //This is the default for command topic.
const COMMAND_OFF:&str="OFF"; //This is the default for command topic.

const STATE_ON:&str="ON"; //This is the default for state topic.
const STATE_OFF:&str="OFF"; //This is the default for state topic.

#[derive(Debug)]
pub enum SwitchState
{
    On,
    Off
}

impl SwitchState
{
    pub fn as_bool(&self)->bool{
        match self
        {
            Self::On=>true,
            Self::Off=>false
        }
    }

    pub fn from_bool(val:bool)->Self
    {
        match val
        {
            true=>Self::On,
            false=>Self::Off
        }
    }
}

impl std::ops::Not for SwitchState
{
    type Output = SwitchState;

    fn not(self) -> Self::Output {
        match self
        {
            Self::On=>Self::Off,
            Self::Off=>Self::On
        }
    }
}

impl HASMQTTState for SwitchState
{
    fn to_payload(&self)->&str {
        match self {
            SwitchState::On=>STATE_ON,
            SwitchState::Off=>STATE_OFF
        }
    }
    
    fn from_payload(payload:bytes::Bytes)->Result<SwitchState, Box<(dyn std::error::Error + 'static)>> {
        match std::str::from_utf8(&payload)
        {
            Ok(str)=>{
                match str
                {
                    COMMAND_ON=>{
                        Ok(SwitchState::On)
                    },
                    COMMAND_OFF=>{
                        Ok(SwitchState::Off)
                    },
                    other=>{
                        Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData,format!("Unexpected payload {}",other))))
                    }
                }
            }
            Err(e)=>{
                eprintln!("Couldn't convert payload {:?} to string.",payload);
                Err(Box::new(e))
            }
        }
    }
}