use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub id: u32,
    pub name: String,
    pub release_year: u32,
    pub user_rate: f32,
    pub genre: String,
}

pub type GamesData = Arc<Mutex<Vec<Game>>>;

pub fn load_data() -> GamesData {
    let games_data: GamesData = Arc::new(Mutex::new(vec![
        Game {
            id: 1,
            name: "Pac-Man".to_string(),
            release_year: 1980,
            genre: "Arcade".to_string(),
            user_rate: 8.6,
        },
        Game {
            id: 2,
            name: "Super Mario Bros.".to_string(),
            release_year: 1985,
            genre: "Platform".to_string(),
            user_rate: 8.56,
        },
    ]));
    games_data
}
