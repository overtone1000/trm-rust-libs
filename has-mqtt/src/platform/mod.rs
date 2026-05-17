use bytes::Bytes;
use rumqttc::ClientError;

use crate::{component::HomeAssistantDeviceComponent, mqtt_client::{EventHandler, EventHandlers, HASMQTTClient}, platform::{configs::availability::Availability, empty::Empty}};

pub mod empty;
pub mod configs;
pub mod switch;
pub mod text;

const COMMAND_TOPIC_TAIL:&str="/set";
const STATE_TOPIC_TAIL:&str="/cmd";

pub trait Component
{
    fn get_platform()->String;
    fn make_empty()->HomeAssistantDeviceComponent{
        HomeAssistantDeviceComponent::Empty(Empty::new(Self::get_platform()))
    }

    fn name(&self)->&str;
    fn availability(&self)->&Availability;
    fn command_topic(&self)->&str;
    fn state_topic(&self)->&str;
    fn connect_handlers(&self)->EventHandlers;
    
    async fn connect(&self, has_client:&HASMQTTClient)->Result<EventHandlers,ClientError>
    {
        println!("Connecting {}",self.name());
        self.availability().set_availability(has_client, true);
        
        match has_client.get_client().subscribe(self.command_topic().clone(), rumqttc::QoS::AtLeastOnce).await
        {
            Ok(_)=>{},
            Err(e)=>{
                return Err(e);
            }
        };

        Ok(self.connect_handlers())
    }
}

pub trait HASMQTTState
{
    fn to_payload(&self)->&str;
    fn from_payload(payload:Bytes)->Result<Self,Box<dyn std::error::Error>> where Self: Sized;
}

struct CommandHandler<T:HASMQTTState>
{
    handle_state_change:Box<dyn Fn(T)->Option<T>>, //should return the resultant state for updating the state
    state_topic:String, //needs a copy of this
}

impl <T:HASMQTTState> CommandHandler<T>
{
    fn set_state(&self, has_client:&HASMQTTClient, state:T)
    {
        has_client.spawn_publish(
            self.state_topic.clone(),
            rumqttc::QoS::AtLeastOnce,
            false,
            state.to_payload().to_string()
        );
    }
}

impl <T:HASMQTTState> EventHandler for CommandHandler<T>
{
    fn handle(&self, payload:Bytes, has_client:&HASMQTTClient) {
        match T::from_payload(payload){
            Ok(input_state)=>{
                match (self.handle_state_change)(input_state)
                {
                    Some(result)=>{self.set_state(has_client,result);}
                    None=>()
                };
            },
            Err(e)=>{
                eprintln!("{:?}",e);
            }
        };
    }
}