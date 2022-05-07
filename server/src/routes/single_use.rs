use actix_web::{get, post, HttpResponse, web};
use mongodb::{bson::{doc, Document}};
use futures::{TryStreamExt, StreamExt};
use serde_json;
use crate::AppState;
use r_shared::class::Class;

#[get("/classes")]
async fn get_classes(app_data: web::Data<AppState>) -> HttpResponse {
    let collection = app_data.db.collection::<Document>("Class");
    // let classes = collection.find(None, None).await.unwrap();
    // println!("classes: {:#?}", classes);
    let cursor = collection.find(None, None).await;
    let mut result = Vec::new();
    match cursor {
        Ok(mut cursor) => {
            let mut is_good = true;
            while let Some(doc) = cursor.next().await {
                println!("{}", doc.clone().unwrap()); 
                match doc {
                    Ok(doc) => result.push(doc),
                    Err(e) => {
                        is_good = false;
                    }
                }
                if !is_good {break;}
            }
            if(!is_good){
                HttpResponse::NotFound().json("No Class found.")
            }else{
                HttpResponse::Ok().json(result)
            }
        },
        Err(e) => HttpResponse::NotFound().json("No class found.")
    }
}