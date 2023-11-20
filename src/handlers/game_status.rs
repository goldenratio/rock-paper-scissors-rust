use actix_web::{post, HttpResponse, Responder};

#[post("/game_status")]
async fn game_status(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("hello game_status!")
}
