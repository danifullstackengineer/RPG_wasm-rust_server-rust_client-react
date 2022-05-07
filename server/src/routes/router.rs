use actix_web::web;
use super::credential;
use super::single_use;
use super::one_time;


pub fn init_user_routes(cfg: &mut web::ServiceConfig){
    cfg.service(credential::register);
    // One Time usage //
    // cfg.service(credential::post_one_time);
}
pub fn init_single_use_routes(cfg: &mut web::ServiceConfig){
    cfg.service(single_use::get_classes);
}
pub fn init_one_time_use_routes(cfg: &mut web::ServiceConfig){
    cfg.service(one_time::upload_image);
}