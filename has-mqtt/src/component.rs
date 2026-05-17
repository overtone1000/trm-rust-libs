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

async fn configure_component<T:Component>(component:&T, has_client:&HASMQTTClient)->Result<Option<EventHandlers>,ClientError>
{
    component.subscribe_future(has_client).await;
    component.availability().set_availability(has_client, true);
    Ok(Some(component.connect_handlers()))
}

impl HomeAssistantDeviceComponent
{
    pub async fn connect(&self, has_client:&HASMQTTClient)->Result<Option<EventHandlers>,ClientError>
    {
        match self
        {
            HomeAssistantDeviceComponent::Empty(_) => Ok(None), //Empty doesn't need to connect
            HomeAssistantDeviceComponent::Switch(component) => {configure_component(component,has_client).await},
            HomeAssistantDeviceComponent::Text(component) => {configure_component(component,has_client).await},
        }
    }
}

