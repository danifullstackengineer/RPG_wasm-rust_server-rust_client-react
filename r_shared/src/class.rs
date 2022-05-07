use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};
use bson::Bson;
// use mongodb::{bson::Bson::Binary};
use super::status::Status;


#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    pub name: ClassName,
    pub initial_status: Status,
    pub description: String,
    // #[serde(serialize_with = "self::serialize_bytes_as_binary")]
    // #[serde(deserialize_with = "self::deserialize_bytes_as_binary")]
    pub image: Vec<u8>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClassName {
    WARRIOR,
    ARCHER,
    MAGE
}

// fn deserialize_bytes_as_binary<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
//     where D: Deserializer<'de>
// {   
//     match Bson::deserialize(deserializer) {
//         Ok(Binary(bytes)) => Ok(bytes.bytes),
//         Ok(..) => Err(Error::invalid_value(Unexpected::Enum, &"Bson::Binary")),
//         Err(e) => Err(e),
//     }
// }
// fn serialize_bytes_as_binary<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     let binary = Bson::Binary( bytes.bytes());
//     binary.serialize(serializer)
// }