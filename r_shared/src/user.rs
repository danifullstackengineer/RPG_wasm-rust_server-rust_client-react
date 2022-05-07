use wasm_bindgen::prelude::*;

use bson::Bson;
// use bson::{spec::BinarySubtype, Bson};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};
use super::{
    status::Status,
    class::Class,
    item::Item
};

const DEFAULT_EXPERIENCE: u64 = 0;
const DEFAULT_LEVEL: u8 = 0;
const DEFAULT_TIME_PLAYED: u16 = 0;
// TODO: Create a default item const
const DEFAULT_ITEM: Vec<Item> = Vec::new();

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    email: String,
    name: Option<String>,
    password: String,
    #[serde(default = "experience_default")]
    experience: u64,
    #[serde(default = "level_default")]
    level: u8,
    #[serde(default = "time_played_default")]
    time_played: u16,
    #[serde(default = "item_default")]
    items: Vec<Item>,
    status: Status,
    class: Class
}

fn experience_default() -> u64 {DEFAULT_EXPERIENCE}
fn level_default() -> u8 {DEFAULT_LEVEL}
fn time_played_default() -> u16 {DEFAULT_TIME_PLAYED}
fn item_default() -> Vec<Item> {DEFAULT_ITEM}

#[wasm_bindgen]
pub enum Keys {
    ArrowUp = 0,
    ArrowDown = 1,
    ArrowLeft = 2,
    ArrowRight = 3,
}