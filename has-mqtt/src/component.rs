use std::collections::HashMap;

use rumqttc::{AsyncClient, ClientError, Event};
use serde::{Deserialize, Serialize};

use crate::{mqtt_client::{EventHandlers, HASMQTTClient}, platform::{Component, empty::Empty, switch::{component::Switch, state::SwitchState}, text::component::Text}};


#[derive(Serialize)]
#[serde(untagged)]
pub enum HomeAssistantDeviceComponent
{
    Empty(Empty),
    Switch(Switch),
    Text(Text)
}

impl HomeAssistantDeviceComponent
{
    pub async fn connect(&self, has_client:&HASMQTTClient)->Result<Option<EventHandlers>,ClientError>
    {
        //Probably a more elegant way to do this...
        match self
        {
            HomeAssistantDeviceComponent::Empty(_) => Ok(None), //Empty doesn't need to connect
            HomeAssistantDeviceComponent::Switch(component) => {
                match component.connect(has_client).await
                {
                    Ok(handlers)=>Ok(Some(handlers)),
                    Err(e)=>{Err(e)}
                }
            },
            HomeAssistantDeviceComponent::Text(component) => {
                match component.connect(has_client).await
                {
                    Ok(handlers)=>Ok(Some(handlers)),
                    Err(e)=>{Err(e)}
                }
            },
        }
    }
}

