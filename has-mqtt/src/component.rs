use rumqttc::{AsyncClient, ClientError};
use serde::{Deserialize, Serialize};

use crate::platform::{empty::Empty, switch::Switch};


#[derive(Serialize,Deserialize,Debug,PartialEq)]
#[serde(untagged)]
pub enum HomeAssistantDeviceComponent
{
    Empty(Empty),
    Switch(Switch)
}

impl HomeAssistantDeviceComponent
{
    pub fn new_switch(
        unique_id:&str
    )->HomeAssistantDeviceComponent
    {
        HomeAssistantDeviceComponent::Switch(
            Switch::new(
                unique_id.to_string()
            )
        )
    }

    pub async fn connect(&self, client:&AsyncClient)->Result<(),ClientError>
    {
        match self
        {
            HomeAssistantDeviceComponent::Empty(_) => Ok(()), //Empty doesn't need to connect
            HomeAssistantDeviceComponent::Switch(switch) => {
                println!("Connecting switch.");
                switch.availability().set_availability(client.clone(), true);
                client.subscribe(switch.get_command_topic(), rumqttc::QoS::AtLeastOnce).await
            },
        }
    }
}

