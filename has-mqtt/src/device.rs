
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::component::HomeAssistantDeviceComponent;

const TOPIC_TAIL:&str="/state";

#[derive(Serialize,Deserialize,Debug,PartialEq)]
struct Device
{
    ids:String, //mandatory
    name:String, //mandatory
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
struct Origin
{
    name:String, //mandatory
    sw:String //mandatory

}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct HomeAssistantDeviceConfiguration
{
    dev:Device,
    o:Origin,
    state_topic:String,
    qos:u16, //should be 2 always
    cmps:HashMap<String,HomeAssistantDeviceComponent>
}

impl HomeAssistantDeviceConfiguration
{
    pub fn new(
        device_id:String,
        device_name:String,
        origin_name:String,
        origin_sw:String,
        //state_topic:String,
        cmps:HashMap<String,HomeAssistantDeviceComponent>
    )->HomeAssistantDeviceConfiguration
    {
        HomeAssistantDeviceConfiguration{
            dev:Device{
                ids:device_id.clone(),
                name:device_name
            },
            o:Origin{
                name:origin_name,
                sw:origin_sw
            },
            state_topic:device_id+TOPIC_TAIL,
            qos:2,
            cmps
        }
    }


    pub fn to_json(&self)->String
    {
        serde_json::to_string(self).expect("Should serialize.")
    }
}
