mod tests;
mod player;
mod utils;
mod ajax;
mod types;
mod logic;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    // #[wasm_bindgen(js_namespace = console, js_name = log)]
    // fn log_many(current: u8, key: Option<Keys>);
}



#[wasm_bindgen]
pub fn handle_login_form(email: String, password: String) -> bool {
    unimplemented!()
}

