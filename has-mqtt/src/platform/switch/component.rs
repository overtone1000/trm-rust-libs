use std::rc::Rc;

use serde::Serialize;

use crate::{component::HomeAssistantDeviceComponent, mqtt_client::{EventHandlers, HASMQTTClient}, platform::{COMMAND_TOPIC_TAIL, CommandHandler, Component, STATE_TOPIC_TAIL, configs::availability::Availability, switch::state::SwitchState}};

//see component types, //https://www.home-assistant.io/integrations/mqtt/
const SWITCH_COMPONENT:&str="switch";

#[derive(Serialize)]
pub struct Switch
{
    p:String, //platform, mandatory, for example "switch" https://www.home-assistant.io/integrations/switch.mqtt/
    //device_class:String, //device class, optional
    command_topic:String, //mandatory, set to none to remove device. Changing this instructs device to change its state.
    state_topic:String, //optional. Changing this informs listeners that the device state is changed.
    unique_id:String, //mandatory with device discovery, set to none to remove device
    name:String,
    availability:Availability,
    #[serde(skip_serializing)]
    command_handler:Rc<CommandHandler<SwitchState>>
}

impl Switch
{
    pub fn new(
        device_id:&str,
        device_name:&str,
        component_id_tag:&str,
        component_name_tag:&str,
        handle_state_change:Box<dyn Fn(SwitchState)->Option<SwitchState>>
    )->HomeAssistantDeviceComponent
    {
        let unique_id=device_id.to_string()+"_"+component_id_tag;
        let name = device_name.to_string()+" "+component_name_tag;

        let state_topic=unique_id.to_string()+STATE_TOPIC_TAIL;

        HomeAssistantDeviceComponent::Switch(
            Switch { 
                p:Self::get_platform(),
                command_topic:unique_id.to_string()+COMMAND_TOPIC_TAIL,
                state_topic:state_topic.clone(),
                unique_id:unique_id.to_string(),
                name:name.to_string(),
                availability:Availability::new(unique_id.to_string()),
                command_handler:Rc::new(CommandHandler{
                    handle_state_change,
                    state_topic:state_topic
                })
            }
        )
    }
}

impl Component for Switch
{
    fn get_platform()->String {
        SWITCH_COMPONENT.to_string()
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