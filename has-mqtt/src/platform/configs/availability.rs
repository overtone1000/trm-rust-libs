use serde::{Deserialize, Serialize};

const TOPIC_TAIL:&str="/availability";
const PAYLOAD_AVAILABLE:&str="online"; //This is the default.
const PAYLOAD_NOT_AVAILABLE:&str="offline"; //This is the default.


#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct Availability{
    //payload_available:String //Just defaults to online
    //payload_not_available:String //Just defaults to offline
    topic:String
    //value_template //optional
}

impl Availability{
    pub fn new(unique_id:String)->Availability
    {
        Availability{
            topic:unique_id+TOPIC_TAIL
        }
    }
}