use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn handle_key_switch(current: u8, key: u8 ) -> u8 {
    return match (current, key) {
        (0, 0) => 2,
        (1, 0) => 0,
        (2, 0) => 1,
        (0, 1) => 1,
        (1, 1) => 2,
        (2, 1) => 0,
        _ => current
    }
}

#[wasm_bindgen]
pub fn handle_enter_switch(current: u8) -> String {
    match current{
        0 => "/login",
        1 => "/register",
        2 => "/docs",
        _ => "/"
    }.to_string()
}