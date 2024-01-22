use actix_web::{post, HttpResponse, Responder};

#[post("/game_events")]
async fn game_events(_req_body: String) -> impl Responder {
    HttpResponse::Ok().body("hello SSE game_events!")
}
