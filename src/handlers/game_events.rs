use actix_web::{Responder, web, get};
use crate::game_event_broadcast::Broadcaster;

#[get("/game_events/{game_id}")]
async fn game_events(path: web::Path<String>, broadcaster: web::Data<Broadcaster>) -> impl Responder {
    let game_id = path.into_inner();
    broadcaster.new_client(game_id.clone()).await
}
