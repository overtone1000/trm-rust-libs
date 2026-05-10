use serde::{Deserialize, Serialize};

use crate::platform::{empty::Empty, switch::Switch};


#[derive(Serialize,Deserialize,Debug,PartialEq)]
#[serde(untagged)]
pub enum HomeAssistantDeviceComponent
{
    Empty(Empty),
    Switch(Switch)
}

impl HomeAssistantDeviceComponent
{
    pub fn new_switch(
        unique_id:&str
    )->HomeAssistantDeviceComponent
    {
        HomeAssistantDeviceComponent::Switch(
            Switch::new(
                unique_id.to_string()
            )
        )
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::{component::HomeAssistantDeviceComponent, device::{HomeAssistantDeviceConfiguration}};

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