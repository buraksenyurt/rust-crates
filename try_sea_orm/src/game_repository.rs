use crate::entity::game;
use sea_orm::*;

pub struct GameRepository;

impl GameRepository {
    pub async fn seed(&self, db: &DatabaseConnection) -> Result<(), DbErr> {
        let games = vec![
            game::ActiveModel {
                title: Set("Pac-Man".to_owned()),
                release_year: Set(1980),
                user_rate: Set(9.5),
                ..Default::default()
            },
            game::ActiveModel {
                title: Set("Donkey Kong".to_owned()),
                release_year: Set(1981),
                user_rate: Set(9.0),
                ..Default::default()
            },
            game::ActiveModel {
                title: Set("Tetris".to_owned()),
                release_year: Set(1984),
                user_rate: Set(9.8),
                ..Default::default()
            },
        ];

        for game in games {
            let existing_game = game::Entity::find()
                .filter(game::Column::Title.eq(game.title.clone().unwrap()))
                .one(db)
                .await?;

            if existing_game.is_none() {
                game::Entity::insert(game.clone()).exec(db).await?;
                println!("Inserted: {}", game.title.unwrap());
            } else {
                println!("Skipped: {}", game.title.unwrap());
            }
        }

        Ok(())
    }

    pub async fn get_all(&self, db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        let games = game::Entity::find().all(db).await?;

        for game in games {
            println!(
                "ID: {}, Title: {}, Release Year: {}, User Rate: {}",
                game.id, game.title, game.release_year, game.user_rate
            );
        }

        Ok(())
    }
}
