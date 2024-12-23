/*
    Program : En Sevdiğim Oyunlar
    Amaç    : actix-web kullanımının örneklenmesi

    Senaryo : Sevilen tarihi bilgisayar oyunlarının sunulduğu basit bir rest servisi.

    Fonksiyonellikler :

    Oyun Ekleme
    Oyun Listeleme

    Detaylar :

    Sunucuyu çalıştırdıktan sonra aşağıdaki gibi talepler yollanır.

    curl http://127.0.0.1:5555/games

    curl -X POST http://127.0.0.1:5555/games \
     -H "Content-Type: application/json" \
     -d '{"id":3,"name":"Tetris","release_year":1984,"genre":"Puzzle","user_rate":8.56}'

     ya da postman ile aynı içerik gönderilebilir

    Çalıştırma :

    cargo run
*/

mod bare;
mod equipped;
mod game;

//#[tokio::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // bare::run().await?;
    equipped::run().await?;

    Ok(())
}
