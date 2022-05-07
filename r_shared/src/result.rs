use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type AnyType;
}

#[wasm_bindgen]
pub struct ClientSuccess {
    success: bool,
    message: Option<String>,
    data: Option<AnyType>
}

impl ClientSuccess{
    pub fn new(success: bool, message: Option<String>, data: Option<AnyType>) -> Self {
        Self {
            success,
            message,
            data
        }
    }
}