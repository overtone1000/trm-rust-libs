use bytes::Bytes;
use rumqttc::tokio_rustls::client;
use rumqttc::{AsyncClient, ClientError, Event, EventLoop, MqttOptions, QoS};
use tokio::{task, time};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;
use std::error::Error;

use crate::component::HomeAssistantDeviceComponent;
use crate::device::{self, HomeAssistantDeviceConfiguration};

pub const DEFAULT_DISCOVERY_PREFIX:&str="homeassistant";

pub trait EventHandler
{
    fn handle(&self, payload:Bytes, has_client:&HASMQTTClient);
}

pub type EventHandlers = HashMap<String,Rc<dyn EventHandler>>;

//#[derive(Clone)]
pub struct HASMQTTClient
{
    discovery_prefix:String,
    client:AsyncClient,
    eventloop:EventLoop,
    object_id:String,
    device_configuration:HomeAssistantDeviceConfiguration
}

impl HASMQTTClient
{
    pub async fn start(
        client_id:&str, //https://www.home-assistant.io/integrations/mqtt/#mqtt-discovery
        server_url:&str,
        server_port:u16,
        discovery_prefix:&str, //https://www.home-assistant.io/integrations/mqtt/
        object_id:&str,
        device_configuration:HomeAssistantDeviceConfiguration
    )->Result<HASMQTTClient,Box<dyn std::error::Error + Send + Sync>>
    {
        let (client, eventloop)=Self::initialize(
            client_id,
            server_url,
            server_port
        ).await;

        Ok(
            HASMQTTClient {
                discovery_prefix:discovery_prefix.to_string(),
                client,
                eventloop,
                object_id:object_id.to_string(),
                device_configuration
            }
        )
    }

    pub fn get_client(&self)->&AsyncClient{&self.client}

    pub fn spawn_publish<V,S>(&self, topic:S, qos:QoS, retain:bool, payload:V)
    where
    S: Into<String>+std::marker::Send+'static,
    V: Into<Vec<u8>>+std::marker::Send+'static
    {
        let client = self.client.clone();
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

    async fn initialize(
        client_id:&str,
        server_url:&str,
        server_port:u16
    )->(AsyncClient,EventLoop)
    {
        let mut mqttoptions = MqttOptions::new(client_id, server_url,server_port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

        (client,eventloop)
    }

    async fn handle_connection(&self)->HashMap<String,Rc<dyn EventHandler>>
    {
        self.device_configuration.publish_discovery(
            &self.client, 
            self.discovery_prefix.to_string(),
            &self.object_id,
        ).await;

        self.device_configuration.connect_components(&self).await
    }

    pub async fn run(mut self)->Result<(),Box<dyn std::error::Error + Send + Sync>>
    {
        let mut handlers:Option<HashMap<String,Rc<dyn EventHandler>>>=None;
        loop {
            let notification = self.eventloop.poll().await.unwrap();

            match notification
            {
                Event::Incoming(packet) => {
                    match packet
                    {
                        rumqttc::Packet::Publish(publish) => {
                            match &handlers
                            {
                                Some(handlers)=>{
                                    match handlers.get(&publish.topic)
                                    {
                                        Some(handler) => {
                                            handler.handle(publish.payload,&self);
                                        },
                                        None => (),
                                    }
                                },
                                None=>{
                                    eprintln!("No handlers.");
                                }
                            }
                        },
                        rumqttc::Packet::ConnAck(conn_ack)=>
                        {
                            if conn_ack.code==rumqttc::ConnectReturnCode::Success
                            {
                                handlers=Some(self.handle_connection().await);
                            }
                            else {
                                eprintln!("Failed to connect. {:?}", conn_ack);
                            }
                        }
                        unhandled=>{
                            println!("{:?}",unhandled)
                        }
                    }
                },
                Event::Outgoing(_outgoing) => (),
            }
        }
    }
}