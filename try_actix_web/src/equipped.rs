use crate::game::*;
use actix_web::web::{Data, Json};
use actix_web::*;

pub async fn run() -> std::io::Result<()> {
    let repo: GamesData = load_data();
    let data = web::Data::new(repo);
    let server_address = "127.0.0.1:5555";
    println!("Server is running at http://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/games", web::get().to(get_games))
            .route("/games", web::post().to(add_game))
    })
    .bind(server_address)?
    .run()
    .await?;

    Ok(())
}

async fn get_games(repo: Data<GamesData>) -> impl Responder {
    let games = repo.lock().await;
    web::Json(games.clone())
}

async fn add_game(game: Json<Game>, repo: Data<GamesData>) -> impl Responder {
    let mut games = repo.lock().await;
    games.push(game.into_inner());
    HttpResponse::Ok().body("Game added!")
}
