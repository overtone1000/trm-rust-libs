use rumqttc::tokio_rustls::client;
use rumqttc::{AsyncClient, ClientError, EventLoop, MqttOptions, QoS};
use tokio::{task, time};
use std::collections::HashMap;
use std::time::Duration;
use std::error::Error;

use crate::component::HomeAssistantDeviceComponent;
use crate::device::HomeAssistantDeviceConfiguration;

pub const DEFAULT_DISCOVERY_PREFIX:&str="homeassistant";

#[derive(Clone)]
pub struct HASMQTTClient
{
    client_id:String, //https://www.home-assistant.io/integrations/mqtt/#mqtt-discovery
    server_url:String,
    server_port:u16,
    discovery_prefix:String, //https://www.home-assistant.io/integrations/mqtt/
}

impl HASMQTTClient
{
    pub fn new(
        client_id:&str,
        server_url:&str,
        server_port:u16,
        discovery_prefix:&str
    )->HASMQTTClient
    {
        HASMQTTClient {
            client_id:client_id.to_string(),
            server_url:server_url.to_string(),
            server_port:server_port,
            discovery_prefix:discovery_prefix.to_string(), 
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

    pub async fn initialize(&self)->Result<(AsyncClient,EventLoop),Box<dyn std::error::Error + Send + Sync>>
    {
        let mut mqttoptions = MqttOptions::new(&self.client_id, &self.server_url,self.server_port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

        Ok((client,eventloop))
    }

    pub async fn run(&self, object_id:&str, device_configuration:HomeAssistantDeviceConfiguration)->Result<(),Box<dyn std::error::Error + Send + Sync>>
    {
        let (client, mut eventloop)=match self.initialize().await
        {
            Ok(result)=>result,
            Err(e)=>{
                eprintln!("Couldn't initialize mqtt");
                return Err(e);
            }
        };

        device_configuration.publish_discovery(
            &client, 
            self.discovery_prefix.to_string(),
            object_id,
        );

        loop {
            let notification = eventloop.poll().await.unwrap();
            println!("Received = {:?}", notification);
        }
    }
}