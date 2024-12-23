/*
    Program : Games Almanac
    Amaç    : sea-orm crate kullanımının örneklenmesi

    Senaryo : Popüler oyunlara ait bilgilerin tutulduğu Sqlite türünden bir veritabanını,
              sea-orm isimli ORM aracını kullanarak oluşturmak, veri ekleme, listeleme gibi basit
              fonksiyonellikleri gerçekleştirmek.

    Fonksiyonellikler :

    Oyun Ekleme
    Oyun Listeleme

    Detaylar : sea-orm haricinde, asenkron fonksiyonellik desteği için tokio ve çevre değişkenlerini
    okumak için dotenvy küfesinden yararlanılmaktadır.

    #00 ORM Command Line Interface desteği için
    cargo install sea-orm-cli

    #01 Migration klasörünün oluşturulması
    sea-orm-cli migrate init

    #02 Migration tanımlanması
    sea-orm-cli migrate generate create_games_table

    #03 Migration planını çalıştırmak
    sea-orm-cli migrate up

    #04 Model nesnelerinin oluşturulması
    sea-orm-cli generate entity -o sr/entity

    Çalıştırma :

    cargo run
*/
mod entity;
mod game_repository;

use crate::game_repository::GameRepository;
use dotenvy::dotenv;
use sea_orm::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(database_url).await?;
    let repository = GameRepository;
    repository.seed(&db).await?;
    repository.get_all(&db).await?;

    Ok(())
}
