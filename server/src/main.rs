use actix_web::{get, web, App, HttpServer, Responder, HttpRequest, HttpResponse, post};
use actix_cors::Cors;
mod db;
mod routes;

#[derive(Debug)]
struct AppState {
    db: mongodb::Database,
    client: mongodb::Client
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();
    let (client_res, db_res) = db::create_conn().await.unwrap();
    HttpServer::new(move|| {
        let cors = Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "PUT"])
                    .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState { client: client_res.clone(), db: db_res.clone() }))
            .service(web::scope("/user").configure(routes::router::init_user_routes))
            .service(web::scope("/single_use").configure(routes::router::init_single_use_routes))
            .service(web::scope("/one_time_use").configure(routes::router::init_one_time_use_routes))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
