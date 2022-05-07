
use actix_web::http::StatusCode;
use actix_web::{post, get, web, HttpRequest, HttpResponse};
use mongodb::{Client};
use mongodb::bson::{doc, Document, Bson, bson};
use mongodb::bson;
use serde::{Serialize, Deserialize};
use std::os;
use std::io::{BufReader, self, BufWriter, Read, Bytes};
use std::fs::File;
use futures::{TryStreamExt, StreamExt};

use crate::AppState;

#[derive(Deserialize, Serialize, Debug)]
struct PathEnum {
    path: String
}

// continue posting images

#[post("/images_upload")]
async fn upload_image(args: web::Json<PathEnum>, app_data: web::Data<AppState>) -> HttpResponse {
    println!("Path is: {}", args.path);
    let f = File::open(format!("../src/assets/{}", args.path)).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    // reader.read_exact(&mut buffer).unwrap();
    let collection = app_data.db.collection::<Document>("Class");
    // let mut bin = std::io::Cursor::new(buffer);
    // let mut doc = mongodb::bson::Document::from_bufD
    println!("Error is: {:#?}", buffer);
    
    let doc = doc!{"key": "value"};
    let mut byte_array: Vec<u8> = vec![1,2,3,4,5];
    doc.to_writer(&mut byte_array).unwrap();

    //  let cursor = collection.find_one_and_update(doc!{"_id": "62741ba21486b244ffe8f60c"}, doc!{"$set": {"image": doc}}, None).await.unwrap();
    let mut cursor = collection.find(doc!{"_id": "62741ba21486b244ffe8f60c"}, None).await.unwrap();
    while let Some(doc) = &cursor.next().await {
        println!("{}", doc.clone().unwrap())
    }
    // println!("{:?}", cursor);
    // match cursor {
    //     Some(c) => println!("Found"),
    //     None => println!("Not found!"),
    // }
    HttpResponse::Ok().json("Good!")
}