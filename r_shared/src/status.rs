use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub py_attack: u16,
    pub mg_attack: u16,
    pub defence: u16,
    pub health: u16,
    pub speed: u16,
}