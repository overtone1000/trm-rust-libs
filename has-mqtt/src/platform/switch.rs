use std::rc::Rc;

use bytes::Bytes;
use rumqttc::{AsyncClient, ClientError, Event};
use serde::{Deserialize, Serialize};

use crate::{mqtt_client::{EventHandler, EventHandlers, HASMQTTClient}, platform::{Platform, configs::availability::Availability}};

//see component types, //https://www.home-assistant.io/integrations/mqtt/
const SWITCH_COMPONENT:&str="switch";
const COMMAND_TOPIC_TAIL:&str="/set";
const STATE_TOPIC_TAIL:&str="/state";

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

#[derive(Serialize)]
pub struct Switch
{
    p:String, //platform, mandatory, for example "switch" https://www.home-assistant.io/integrations/switch.mqtt/
    //device_class:String, //device class, optional
    command_topic:String, //mandatory, set to none to remove device. Changing this instructs device to change its state.
    state_topic:String, //optional. Changing this informs listeners that the device state is changed.
    unique_id:String, //mandatory with device discovery, set to none to remove device
    availability:Availability,
    #[serde(skip_serializing)]
    switch_handler:Rc<SwitchCommandHandler>
}

struct SwitchCommandHandler
{
    handle_state_change:fn(SwitchState)->SwitchState, //should return the resultant state
    state_topic:String, //needs a copy of this
}

impl Switch
{
    pub fn new(unique_id:String, handle_state_change:fn(SwitchState)->SwitchState)->Switch
    {
        let state_topic=unique_id.clone()+STATE_TOPIC_TAIL;

        Switch { 
            p:Self::get_platform(),
            command_topic:unique_id.clone()+COMMAND_TOPIC_TAIL,
            state_topic:state_topic.clone(),
            unique_id:unique_id.clone(),
            availability:Availability::new(unique_id),
            switch_handler:Rc::new(SwitchCommandHandler{
                handle_state_change,
                state_topic:state_topic
            })
        }
    }

    fn availability(&self)->&Availability{&self.availability}

    pub async fn connect(&self, has_client:&HASMQTTClient)->Result<EventHandlers,ClientError>
    {
        println!("Connecting switch.");
        self.availability().set_availability(has_client, true);
        
        match has_client.get_client().subscribe(self.command_topic.clone(), rumqttc::QoS::AtLeastOnce).await
        {
            Ok(_)=>{},
            Err(e)=>{
                return Err(e);
            }
        };

        let mut handlers = EventHandlers::new();
        handlers.insert(
            self.command_topic.clone(),
            self.switch_handler.clone()
        );

        Ok(handlers)
    }
}

impl Platform for Switch
{
    fn get_platform()->String {
        SWITCH_COMPONENT.to_string()
    }
}

impl SwitchCommandHandler
{
    fn set_state(&self, has_client:&HASMQTTClient, state:SwitchState)
    {
        let state_payload = match state {
            SwitchState::On=>STATE_ON,
            SwitchState::Off=>STATE_OFF
        };

        has_client.spawn_publish(
            self.state_topic.clone(),
            rumqttc::QoS::AtLeastOnce,
            false,
            state_payload
        );
    }
}

impl EventHandler for SwitchCommandHandler
{
    fn handle(&self, payload:Bytes, has_client:&HASMQTTClient) {
        let input:SwitchState = match std::str::from_utf8(&payload)
        {
            Ok(str)=>{
                match str
                {
                    COMMAND_ON=>{
                        SwitchState::On
                    },
                    COMMAND_OFF=>{
                        SwitchState::Off
                    },
                    other=>{
                        eprintln!("Unexpected payload {}", other);
                        return;
                    }
                }
            }
            Err(e)=>{
                eprintln!("Couldn't convert payload {:?} to string.",payload);
                eprintln!("{}",e);
                return;
            }
        };

        let output_state = (self.handle_state_change)(input);

        self.set_state(has_client,output_state);        
    }
}