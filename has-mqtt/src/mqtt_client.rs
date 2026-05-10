use rumqttc::{AsyncClient, ClientError, EventLoop, MqttOptions, QoS};
use tokio::{task, time};
use std::collections::HashMap;
use std::time::Duration;
use std::error::Error;

use crate::component::HomeAssistantDeviceComponent;
use crate::device::HomeAssistantDeviceConfiguration;

const NODE_ID:&str="faux_show";


#[derive(Clone)]
pub struct HASMQTTClient
{
    object_id:String, //https://www.home-assistant.io/integrations/mqtt/#mqtt-discovery
    server_url:String,
    server_port:u16,
    discovery_prefix:String, //https://www.home-assistant.io/integrations/mqtt/
}

impl HASMQTTClient
{
    pub fn new()->HASMQTTClient
    {
        HASMQTTClient {
            object_id:"mqtt_testing".to_string(),
            server_url:"10.10.10.10".to_string(),
            server_port:1883,
            discovery_prefix:"homeassistant".to_string(), 
        }
    }

    pub fn spawn_publish<V,S>(client:AsyncClient, topic:S, qos:QoS, retain:bool, payload:V)
    where
    S: Into<String>+std::marker::Send+'static,
    V: Into<Vec<u8>>+std::marker::Send+'static
    {
        task::spawn(async move {
            match client.publish(topic,qos,retain,payload).await
            {
                Ok(_)=>(),
                Err(e)=>{
                    eprintln!("Publish error: {:?}",e);
                }
            }
        });
    }

    fn get_device_config()->String
    {
        let mut cmps:HashMap<String,HomeAssistantDeviceComponent>=HashMap::new();
        
        cmps.insert(
            "test_component_1".to_string(),
            HomeAssistantDeviceComponent::new_switch(
                "test_component_unique_id"
            )
        );

        let config=HomeAssistantDeviceConfiguration::new(
            "test_device_id".to_string(),
            "Test Device Name".to_string(),
            "Test Origin Name".to_string(),
            "1.2.3".to_string(),
            cmps
        );

        config.to_json()
    }

    pub async fn initialize(&self)->Result<(AsyncClient,EventLoop),Box<dyn std::error::Error + Send + Sync>>
    {
        let mut mqttoptions = MqttOptions::new(&self.object_id, &self.server_url,self.server_port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

        Ok((client,eventloop))
    }

    fn get_discovery_topic(&self)->String
    {
        let discovery_topic=self.discovery_prefix.clone()+"/device/"+NODE_ID+"/"+&self.object_id+"/config";

        //client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap(); //Subscribe like this and then receive notifications on eventloop below
        //Self::async_publish(client,"hello/rumqtt", QoS::AtLeastOnce, false, "howdy"); //publish like this

        println!("Discovery topic: {}", discovery_topic);

        discovery_topic
    }

    fn publish_discovery_spawn(client:&AsyncClient, discovery_topic:String, device_config:String)->()
    {
        Self::spawn_publish(client.clone(),discovery_topic, QoS::AtLeastOnce, true, device_config);
    }

    pub async fn run(&self)->Result<(),Box<dyn std::error::Error + Send + Sync>>
    {
        let (client, mut eventloop)=match self.initialize().await
        {
            Ok(result)=>result,
            Err(e)=>{
                eprintln!("Couldn't initialize mqtt");
                return Err(e);
            }
        };

        Self::publish_discovery_spawn(&client, self.get_discovery_topic(), Self::get_device_config());

        loop {
            let notification = eventloop.poll().await.unwrap();
            println!("Received = {:?}", notification);
        }
    }
}



#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::{component::HomeAssistantDeviceComponent, device::HomeAssistantDeviceConfiguration, mqtt_client::HASMQTTClient, platform::{self, Platform as _}};

    use super::*;

    //This doesn't seem to work. Have to poll event loop.
    //pub async fn publish_discovery_async(mqtt:&MQTTClient, client:&AsyncClient)->Result<(),ClientError>
    //{
    //    MQTTClient::async_publish(client.clone(),mqtt.get_discovery_topic(), QoS::AtLeastOnce, true, MQTTClient::get_device_config()).await
    //}
    
    fn get_device_config()->String
    {
        let mut cmps:HashMap<String,HomeAssistantDeviceComponent>=HashMap::new();
        
        cmps.insert(
            "test_component_1".to_string(),
            HomeAssistantDeviceComponent::new_switch(
                "test_component_unique_id"
            )
        );

        let config=HomeAssistantDeviceConfiguration::new(
            "test_device_id".to_string(),
            "Test Device Name".to_string(),
            "Test Origin Name".to_string(),
            "1.2.3".to_string(),
            cmps
        );

        config.to_json()
    }

    fn get_device_clear_config()->String
    {
        let mut cmps:HashMap<String,HomeAssistantDeviceComponent>=HashMap::new();
        
        cmps.insert(
            "test_component_1".to_string(),
            platform::switch::Switch::make_empty()
        );

        let config=HomeAssistantDeviceConfiguration::new(
            "test_device_id".to_string(),
            "Test Device Name".to_string(),
            "Test Origin Name".to_string(),
            "1.2.3".to_string(),
            cmps
        );

        config.to_json()
    }

    #[tokio::test]
    async fn test_publish_device() {
        let mqtt_client:HASMQTTClient = HASMQTTClient::new();
        let (client,mut event_loop)=mqtt_client.initialize().await.expect("Couldn't initialize mqtt client.");
        HASMQTTClient::publish_discovery_spawn(&client, mqtt_client.get_discovery_topic(), get_device_config());
        
        //Must do it this way. Can't publish separately for some reason.
        loop {
            let notification = event_loop.poll().await.unwrap();
            println!("Received = {:?}", notification);
        }
    }

    #[tokio::test]
    async fn test_clear_device() {
        let mqtt_client:HASMQTTClient = HASMQTTClient::new();
        let (client,mut event_loop)=mqtt_client.initialize().await.expect("Couldn't initialize mqtt client.");
        HASMQTTClient::publish_discovery_spawn(&client, mqtt_client.get_discovery_topic(), get_device_clear_config());
        
        //Must do it this way. Can't publish separately for some reason.
        loop {
            let notification = event_loop.poll().await.unwrap();
            println!("Received = {:?}", notification);
        }
    }
}