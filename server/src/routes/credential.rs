
use actix_web::http::StatusCode;
use actix_web::{post, get, web, HttpRequest, HttpResponse};
use mongodb::{Client};
use mongodb::bson::{doc, Document};
use mongodb::bson;
use serde::{Serialize, Deserialize};
use r_shared::class::ClassName;
use std::borrow::Borrow;

use crate::AppState;

#[derive(Deserialize, Serialize,Debug )]
struct RegisterInfo {
    email: String,
    password: String,
    class: ClassName
}
#[post("/register")]
async fn register(email: web::Json<RegisterInfo>, app_data: web::Data<AppState>) -> HttpResponse {
    println!("{:?}", app_data);
    println!("{:?}", email);
    let collection = app_data.db.collection::<Document>("user");
    let user = collection.find_one(bson::doc! {}, None).await.unwrap();
    println!("User was: {:?}", user);
    unimplemented!()
}
// Only for one time usage // 
// #[post("/one_time")]
// async fn post_one_time(data: web::Json<user::Class>, app_data: web::Data<AppState>) -> HttpResponse {
//         let collection = app_data.db.collection::<Document>("Class");
//         let (name, initial_status, description) = (data.name.to_owned(), data.initial_status.to_owned(), data.description.to_owned());
//         collection.insert_one(bson::doc! {
//             "name": bson::to_bson(&name).unwrap(),
//             "initial_status": bson::to_bson(&initial_status).unwrap(),
//             "description": bson::to_bson(&description).unwrap()
//         }, None).await.unwrap();
//        HttpResponse::Ok().json("Ok!")
// }