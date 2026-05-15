use std::rc::Rc;

use serde::Serialize;

use crate::{mqtt_client::EventHandlers, platform::{COMMAND_TOPIC_TAIL, CommandHandler, Component, STATE_TOPIC_TAIL, configs::availability::Availability}};

//see component types, //https://www.home-assistant.io/integrations/mqtt/
const TEXT_COMPONENT:&str="text";

#[derive(Serialize)]
pub struct Text
{
    p:String, //platform, mandatory, for example "Text" https://www.home-assistant.io/integrations/Text.mqtt/
    //device_class:String, //device class, optional
    command_topic:String, //mandatory, set to none to remove device. Changing this instructs device to change its state.
    state_topic:String, //optional. Changing this informs listeners that the device state is changed.
    unique_id:String, //mandatory with device discovery, set to none to remove device
    name:String,
    availability:Availability,
    #[serde(skip_serializing)]
    command_handler:Rc<CommandHandler<String>>
}

impl Text
{
    pub fn new(unique_id:String, name:String, handle_state_change:Box<dyn Fn(String)->String>)->Text
    {
        let state_topic=unique_id.clone()+STATE_TOPIC_TAIL;

        Text { 
            p:Self::get_platform(),
            command_topic:unique_id.clone()+COMMAND_TOPIC_TAIL,
            state_topic:state_topic.clone(),
            unique_id:unique_id.clone(),
            name,
            availability:Availability::new(unique_id),
            command_handler:Rc::new(CommandHandler{
                handle_state_change,
                state_topic:state_topic
            })
        }
    }
}

impl Component for Text
{
    fn get_platform()->String {
        TEXT_COMPONENT.to_string()
    }
    
    fn name(&self)->&str {
        &self.name
    }
    
    fn availability(&self)->&Availability {
        &self.availability
    }
    
    fn command_topic(&self)->&str {
        &self.command_topic
    }
    
    fn state_topic(&self)->&str {
        &self.state_topic
    }
    
    fn connect_handlers(&self)->EventHandlers {
        let mut handlers = EventHandlers::new();
        handlers.insert(
            self.command_topic.clone(),
            self.command_handler.clone()
        );

        handlers
    }
}