
use std::collections::HashMap;

use has_mqtt::{component::HomeAssistantDeviceComponent, device::HomeAssistantDeviceConfiguration, mqtt_client::{DEFAULT_DISCOVERY_PREFIX, HASMQTTClient}, platform::{self, Platform as _}};

//This doesn't seem to work. Have to poll event loop.
//pub async fn publish_discovery_async(mqtt:&MQTTClient, client:&AsyncClient)->Result<(),ClientError>
//{
//    MQTTClient::async_publish(client.clone(),mqtt.get_discovery_topic(), QoS::AtLeastOnce, true, MQTTClient::get_device_config()).await
//}

const TEST_COMPONENT_1:&str="test_component_1";
const TEST_COMPONENT_UNIQUE_ID:&str="test_component_unique_id";
const TEST_DEVICE_ID:&str="test_device_id";
const TEST_DEVICE_NAME:&str="Test Device Name";
const TEST_ORIGIN_NAME:&str="Test Origin Name";
const TEST_ORIGIN_SW:&str="1.2.3";

const TEST_CLIENT_ID:&str="test_client";
const TEST_URL:&str="10.10.10.10";
const TEST_PORT:u16=1883;
const TEST_OBJECT_ID:&str="test_object_id";

fn get_test_client() ->HASMQTTClient 
{
    HASMQTTClient::new(
        TEST_CLIENT_ID,
        TEST_URL,
        TEST_PORT,
        DEFAULT_DISCOVERY_PREFIX
    )
}

async fn test_publish_device() {
    let mqtt_client:HASMQTTClient = get_test_client();
    let (client,mut event_loop)=mqtt_client.initialize().await.expect("Couldn't initialize mqtt client.");

    let mut cmps:HashMap<String,HomeAssistantDeviceComponent>=HashMap::new();

    let test_switch = HomeAssistantDeviceComponent::Switch(platform::switch::Switch::new(TEST_COMPONENT_UNIQUE_ID.to_string()));

    cmps.insert(
        TEST_COMPONENT_1.to_string(),
        test_switch
    );

    let config=HomeAssistantDeviceConfiguration::new(
        TEST_DEVICE_ID.to_string(),
        TEST_DEVICE_NAME.to_string(),
        TEST_ORIGIN_NAME.to_string(),
        TEST_ORIGIN_SW.to_string(),
        cmps
    );

    println!("Publishing discovery.");
    config.publish_discovery(&client, DEFAULT_DISCOVERY_PREFIX.to_string(), TEST_OBJECT_ID);

    println!("Connecting components.");
    match config.get_component(TEST_COMPONENT_1)
    {
        Some(comp)=>{
            match comp.connect(&client).await
            {
                Ok(_)=>(),
                Err(e)=>{eprintln!("{:?}",e);}
            }
        }
        None=>{eprintln!("Couldn't get component.");}
    }

    //Result of command that needs to be subscribed and acted upon
    //Received = Incoming(Publish(Topic = test_component_unique_id/set, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 2))

    println!("Starting loop.");
    //Must do it this way. Can't publish separately for some reason.
    loop {
        let notification = event_loop.poll().await.unwrap();
        println!("Received = {:?}", notification);
    }
}

#[tokio::main]

async fn test_clear_device() {
    let mqtt_client:HASMQTTClient = get_test_client();
    let (client,mut event_loop)=mqtt_client.initialize().await.expect("Couldn't initialize mqtt client.");
    
    let mut cmps:HashMap<String,HomeAssistantDeviceComponent>=HashMap::new();
    
    cmps.insert(
        TEST_COMPONENT_1.to_string(),
        platform::switch::Switch::make_empty()
    );

    let config=HomeAssistantDeviceConfiguration::new(
        TEST_DEVICE_ID.to_string(),
        TEST_DEVICE_NAME.to_string(),
        TEST_ORIGIN_NAME.to_string(),
        TEST_ORIGIN_SW.to_string(),
        cmps
    );

    config.publish_discovery(&client, DEFAULT_DISCOVERY_PREFIX.to_string(), TEST_OBJECT_ID);
    
    //Must do it this way. Can't publish separately for some reason.
    loop {
        let notification = event_loop.poll().await.unwrap();
        println!("Received = {:?}", notification);
    }
}

#[tokio::main]
async fn main(){
    //test_clear_device();
    test_publish_device().await
}