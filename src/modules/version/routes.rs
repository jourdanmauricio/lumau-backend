use actix_web::{get, web, HttpResponse, Responder};
use log::info;

#[get("/version")]
async fn version() -> impl Responder {
    info!("GET version");
    HttpResponse::Ok().body("v1.0.0")
}

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(version);
}
