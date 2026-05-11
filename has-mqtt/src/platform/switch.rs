use serde::{Deserialize, Serialize};

use crate::platform::{Platform, configs::availability::Availability};

//see component types, //https://www.home-assistant.io/integrations/mqtt/
const SWITCH_COMPONENT:&str="switch";
const COMMAND_TOPIC_TAIL:&str="/set";
const STATE_TOPIC_TAIL:&str="/state";

const PAYLOAD_ON:&str="ON"; //This is the default for command topic.
const PAYLOAD_OFF:&str="OFF"; //This is the default for command topic.

const STATE_ON:&str="ON"; //This is the default for state topic.
const STATE_OFF:&str="OFF"; //This is the default for state topic.

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct Switch
{
    p:String, //platform, mandatory, for example "switch" https://www.home-assistant.io/integrations/switch.mqtt/
    //device_class:String, //device class, optional
    command_topic:String, //mandatory, set to none to remove device. Changing this instructs device to change its state.
    state_topic:String, //optional. Changing this informs listeners that the device state is changed.
    unique_id:String, //mandatory with device discovery, set to none to remove device
    availability:Availability
}

impl Switch
{
    pub fn new(unique_id:String)->Switch
    {
        Switch { 
            p:Self::get_platform(),
            command_topic:unique_id.clone()+COMMAND_TOPIC_TAIL,
            state_topic:unique_id.clone()+STATE_TOPIC_TAIL,
            unique_id:unique_id.clone(),
            availability:Availability::new(unique_id)
        }
    }

    pub fn get_state_topic(&self)->String{self.state_topic.clone()}
    pub fn get_command_topic(&self)->String{self.command_topic.clone()}
    pub fn availability(&self)->&Availability{&self.availability}
}

impl Platform for Switch
{
    fn get_platform()->String {
        SWITCH_COMPONENT.to_string()
    }
}

pub trait SwitchHandler
{
    fn handle_state_change(switch:&Switch)->();
}