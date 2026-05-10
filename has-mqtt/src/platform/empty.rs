use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct Empty
{
    p:String, //platform, mandatory, for example "switch" https://www.home-assistant.io/integrations/switch.mqtt/
}

impl Empty
{
    pub fn new(p:String)->Empty
    {
        Empty { p }
    }
}