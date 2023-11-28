use actix_web::{get, web, HttpResponse, Responder};
use crate::AppState;

#[get("/shutdown")]
async fn shutdown(_state: web::Data<AppState>) -> impl Responder {
    println!("/shutdown");
    // state.shutdown_server();
    HttpResponse::Ok().finish()
}
