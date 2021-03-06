use crate::domain::game;
use crate::domain::model::Model;
use actix_web::{web, HttpResponse, Responder};
use shared::api::start_game;
use std::fs;
use std::sync::Mutex;

/// Responder Objects
/// GET /
pub async fn index() -> impl Responder {
    match fs::read_to_string("../ui/index.html") {
        Ok(index_file) => HttpResponse::Ok().body(index_file),
        Err(_error) => HttpResponse::NotFound().body("File not found"),
    }
}

/// GET /again
pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello World Again")
}

/// GET /games/count, we also pass in the state
pub async fn game_count(model: web::Data<Model>) -> impl Responder {
    HttpResponse::Ok().body(model.games_count().to_string())
}

/// POST /game/create This
/// function will be called from a post request
pub async fn post_game(body: String, mutex: web::Data<Mutex<Model>>) -> impl Responder {
    // Decode the hex message from the server
    // hex decode returns a Result type, needs to match

    match hex::decode(body) {
        Ok(bytes) => match start_game::Request::from_bytes(bytes) {
            Ok(request) => {
                let mut model = mutex.lock().unwrap();

                let new_game = game::init_lobby(request.game_name(), request.host_name());
                let game_id: u64 = model.add_game(new_game);
                match start_game::Response::init(game_id).to_bytes() {
                    Ok(response_bytes) => hex::encode(response_bytes),
                    Err(error) => error.to_string(),
                }
            }
            Err(error) => error.to_string(),
        },
        Err(error) => error.to_string(),
    }
}
