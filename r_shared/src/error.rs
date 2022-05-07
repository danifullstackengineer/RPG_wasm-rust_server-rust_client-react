use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Deserialize, Serialize)]
pub struct ClientError {
    intern_err: Option<bool>,
    extern_err: Option<bool>,
    message: Option<String>
}

impl ClientError {
    pub fn new(intern_err: Option<bool>, extern_err: Option<bool>, message: Option<String>) -> Self {
        Self {
            intern_err,
            extern_err,
            message
        }
    }
}