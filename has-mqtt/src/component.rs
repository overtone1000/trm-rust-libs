use std::collections::HashMap;

use rumqttc::{AsyncClient, ClientError, Event};
use serde::{Deserialize, Serialize};

use crate::{mqtt_client::{EventHandlers, HASMQTTClient}, platform::{empty::Empty, switch::{Switch, SwitchState}}};


#[derive(Serialize)]
#[serde(untagged)]
pub enum HomeAssistantDeviceComponent
{
    Empty(Empty),
    Switch(Switch)
}

impl HomeAssistantDeviceComponent
{
    pub fn new_switch(
        unique_id:&str,
        handle_state_change:Box<dyn Fn(SwitchState)->SwitchState>
    )->HomeAssistantDeviceComponent
    {
        HomeAssistantDeviceComponent::Switch(
            Switch::new(
                unique_id.to_string(),
                handle_state_change
            )
        )
    }

    pub async fn connect(&self, has_client:&HASMQTTClient)->Result<Option<EventHandlers>,ClientError>
    {
        match self
        {
            HomeAssistantDeviceComponent::Empty(_) => Ok(None), //Empty doesn't need to connect
            HomeAssistantDeviceComponent::Switch(switch) => {
                match switch.connect(has_client).await
                {
                    Ok(handlers)=>Ok(Some(handlers)),
                    Err(e)=>{Err(e)}
                }
            },
        }
    }
}

