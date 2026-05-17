
use std::collections::HashMap;

use rumqttc::{AsyncClient, ClientError, QoS};
use serde::{Deserialize, Serialize};

use crate::{component::HomeAssistantDeviceComponent, mqtt_client::{EventHandlers, HASMQTTClient}};

const TOPIC_TAIL:&str="/state";

#[derive(Serialize)]
struct Device
{
    ids:String, //mandatory
    name:String, //mandatory
}

#[derive(Serialize)]
struct Origin
{
    name:String, //mandatory
    sw:String //mandatory

}

#[derive(Serialize)]
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

    fn get_discovery_topic(&self, discovery_prefix:String, object_id:&str)->String
    {
        let discovery_topic=discovery_prefix+"/device/"+object_id+"/config";

        //client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap(); //Subscribe like this and then receive notifications on eventloop below
        //Self::async_publish(client,"hello/rumqtt", QoS::AtLeastOnce, false, "howdy"); //publish like this

        println!("Discovery topic: {}", discovery_topic);

        discovery_topic
    }

    pub async fn publish_discovery(&self, client:&AsyncClient, discovery_prefix:String, object_id:&str)->()
    {
        let discovery_topic=self.get_discovery_topic(discovery_prefix, object_id);
        
        //DON'T do this asynchronously in a separate process. Instead, await (as below). Need to wait for this to complete to connect components!
        //HASMQTTClient::spawn_publish(client.clone(),discovery_topic, QoS::AtLeastOnce, true, self.to_json());

         match client.publish(discovery_topic,QoS::AtLeastOnce,true,self.to_json()).await
        {
            Ok(_)=>(),
            Err(e)=>{
                eprintln!("Publish error: {:?}",e);
            }
        }
    }

    fn to_json(&self)->String
    {
        serde_json::to_string(self).expect("Should serialize.")
    }

    pub async fn connect_components(&self, has_client: &HASMQTTClient)->EventHandlers
    {
        let mut device_handlers=EventHandlers::new();

        for (_key,component) in &self.cmps
        {
            match component.connect(has_client).await
            {
                Ok(component_handlers)=>{
                    match component_handlers
                    {
                        Some(component_handlers)=>{
                            for (key, val) in component_handlers
                            {
                                device_handlers.insert(key,val);
                            }
                        },
                        None=>()
                    }
                },
                Err(e)=>{
                    eprintln!("Error connecting component. {:?}",e);
                }
            }
        }

        device_handlers
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::platform::switch::{component::Switch, state::SwitchState};

    use super::*;

    fn check_serialization(device_config: &HomeAssistantDeviceConfiguration) {
        println!("Serialization test:");
        let serialized = device_config.to_json();
        println!("   {}", serialized);
    }

    #[test]
    fn serialization() {

        let state_change = |new_state:SwitchState|->Option<SwitchState>{
            println!("Got state change: {:?}", new_state);
            Some(new_state)
        };

        let mut cmps:HashMap<String,HomeAssistantDeviceComponent>=HashMap::new();
        cmps.insert(
            "test_component_1".to_string(),
            Switch::new(
                "device_id",
                "Device Name",
                "test_component_unique_id",
                "Test Switch",
                Box::new(state_change)
            )
        );

        check_serialization(
&HomeAssistantDeviceConfiguration::new(
                "test_device_id".to_string(),
                "Test Device Name".to_string(),
                "Test Origin Name".to_string(),
                "1.2.3(test)".to_string(),
                cmps
            )
        );
    }
}