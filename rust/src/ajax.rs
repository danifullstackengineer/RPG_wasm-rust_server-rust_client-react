use wasm_bindgen::__rt::IntoJsResult;
use wasm_bindgen::prelude::*;
// use super::response::Response as ResultQuery;
use r_shared::result::ClientSuccess;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
// use super::response::Error as ErrorQuery;
use r_shared::error::ClientError;
use super::utils::{check_regex_email, check_regex_password, check_same_pw};
use r_shared::class::Class;


#[wasm_bindgen]
pub async fn fetch_classes() -> Result<JsValue, JsValue> {

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("http://localhost:5000/single_use/classes");

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        // web_sys::console::log_1(&resp_value.clone().into());

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    // web_sys::console::log_1(&json);

    let class_info: Result<Vec<Class>, serde_json::error::Error> = json.into_serde();


    // Send the `Branch` struct back to JS as an `Object`.
    Ok(JsValue::from_serde(&class_info.unwrap()).unwrap())
}


// TODO: Finish implementing the register fn
#[wasm_bindgen]
pub async fn verify_and_register(email: String, password: String, repassword: String, class_r: u8) -> Result<ClientSuccess, ClientError> {

    let r1 = check_regex_email(&email).await;
    let r2 = check_regex_password(&password).await;
    let r3 = check_same_pw(&r2.returns, &repassword).await;

    if !r1.truth {return Err(ClientError::new(Some(true), None, Some("Please use a valid email address.".to_string())))}
    if !r2.truth {return Err(ClientError::new(Some(true),None, Some( "Please use a valid password.".to_string())))}
    if !r3.truth {return Err(ClientError::new(Some(true), None, Some("Passwords must match.".to_string())))}
    if !(class_r == 0 || class_r == 1 || class_r == 2) {return Err(ClientError::new(Some(true), None, Some("Please choose a valid class.".to_string())))}

    let mut opts = RequestInit::new()
                                    .method("POST")
                                    .mode(RequestMode::Cors);

                                    //todo: change the values
    Ok(ClientSuccess::new(true, None, None))
}