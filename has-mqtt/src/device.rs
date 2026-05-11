
use std::collections::HashMap;

use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};

use crate::{component::HomeAssistantDeviceComponent, mqtt_client::HASMQTTClient};

const TOPIC_TAIL:&str="/state";

#[derive(Serialize,Deserialize,Debug,PartialEq)]
struct Device
{
    ids:String, //mandatory
    name:String, //mandatory
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
struct Origin
{
    name:String, //mandatory
    sw:String //mandatory

}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
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

    pub fn publish_discovery(&self, client:&AsyncClient, discovery_prefix:String, object_id:&str)->()
    {
        let discovery_topic=self.get_discovery_topic(discovery_prefix, object_id);
        HASMQTTClient::spawn_publish(client.clone(),discovery_topic, QoS::AtLeastOnce, true, self.to_json());
    }

    fn to_json(&self)->String
    {
        serde_json::to_string(self).expect("Should serialize.")
    }

    pub fn get_component(&self, key:&str)->Option<&HomeAssistantDeviceComponent>
    {
        self.cmps.get(key)
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    fn check_serialization(device_config: &HomeAssistantDeviceConfiguration) {
        println!("Serialization test:");
        let serialized = device_config.to_json();
        println!("   {}", serialized);
        let deserialized: HomeAssistantDeviceConfiguration = serde_json::from_str(&serialized).expect("Should deserialize.");
        println!("   {:?}", deserialized);

        //The results won't be equal because they're untagged.
        assert_ne!(*device_config,deserialized)        
    }

    #[test]
    fn serialization() {

        let mut cmps:HashMap<String,HomeAssistantDeviceComponent>=HashMap::new();
        cmps.insert(
            "test_component_1".to_string(),
            HomeAssistantDeviceComponent::new_switch(
                "test_component_unique_id")
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