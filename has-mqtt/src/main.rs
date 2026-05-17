
use std::collections::HashMap;

use has_mqtt::{component::HomeAssistantDeviceComponent, device::{self, HomeAssistantDeviceConfiguration}, mqtt_client::{DEFAULT_DISCOVERY_PREFIX, HASMQTTClient}, platform::{self, Component as _, switch::{component::Switch, state::SwitchState}}};

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

async fn get_test_client(device_configuration:HomeAssistantDeviceConfiguration) ->HASMQTTClient 
{
    HASMQTTClient::start(
        TEST_CLIENT_ID,
        TEST_URL,
        TEST_PORT,
        DEFAULT_DISCOVERY_PREFIX,
        TEST_OBJECT_ID,
        device_configuration
    ).await
}

async fn test_publish_device() {
    

    let state_change = |new_state:SwitchState|->Option<SwitchState>{
        println!("Got state change: {:?}", new_state);
        Some(new_state)
    };

    let mut cmps:HashMap<String,HomeAssistantDeviceComponent>=HashMap::new();

    let test_switch: HomeAssistantDeviceComponent = Switch::new(
        TEST_COMPONENT_UNIQUE_ID,
        "Test Switch",
        Box::new(state_change)
    );

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

    let mqtt_client:HASMQTTClient = get_test_client(config).await;
    mqtt_client.run().await.expect("Shouldn't ever finish.");
    
}

async fn test_clear_device() {
    let mut cmps:HashMap<String,HomeAssistantDeviceComponent>=HashMap::new();
    
    cmps.insert(
        TEST_COMPONENT_1.to_string(),
        platform::switch::component::Switch::make_empty()
    );

    let config=HomeAssistantDeviceConfiguration::new(
        TEST_DEVICE_ID.to_string(),
        TEST_DEVICE_NAME.to_string(),
        TEST_ORIGIN_NAME.to_string(),
        TEST_ORIGIN_SW.to_string(),
        cmps
    );

    println!("Creating client.");
    let mqtt_client:HASMQTTClient = get_test_client(config).await;
    println!("Running client.");
    mqtt_client.run().await.expect("Shouldn't ever finish.");
}

#[tokio::main]
async fn main(){
    //test_clear_device().await;
    test_publish_device().await
}