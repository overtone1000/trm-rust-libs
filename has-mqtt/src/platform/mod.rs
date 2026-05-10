use crate::{component::HomeAssistantDeviceComponent, platform::empty::Empty};

pub mod empty;
pub mod configs;
pub mod switch;

pub trait Platform
{
    fn get_platform()->String;
    fn make_empty()->HomeAssistantDeviceComponent{
        HomeAssistantDeviceComponent::Empty(Empty::new(Self::get_platform()))
    }
}