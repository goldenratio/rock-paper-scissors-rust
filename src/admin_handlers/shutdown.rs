use crate::app_state::AppState;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/shutdown")]
async fn shutdown(_state: web::Data<AppState>) -> impl Responder {
    println!("/shutdown");
    // state.shutdown_server();
    HttpResponse::NoContent().finish()
}
