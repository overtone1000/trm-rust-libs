use std::thread;

use rumqttc::AsyncClient;
use serde::{Deserialize, Serialize};

use crate::mqtt_client::HASMQTTClient;

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

    pub fn set_availability(&self,has_client:&HASMQTTClient,is_available:bool)->(){
        let payload = match is_available
        {
            true=>PAYLOAD_AVAILABLE,
            false=>PAYLOAD_NOT_AVAILABLE
        };
        println!("Setting availability {} to {}",self.topic,payload);
        
        //Use a delay to wait for availability topics to be ready on server
        has_client.spawn_publish(self.topic.clone(), rumqttc::QoS::AtLeastOnce, false, payload, Some(std::time::Duration::from_secs(3)));
    }
}