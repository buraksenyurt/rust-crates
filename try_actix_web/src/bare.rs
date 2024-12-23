use crate::game::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5555").await?;
    let games_data = load_data();

    println!("Server running on http://127.0.0.1:5555");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let games = games_data.clone();

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let _ = socket.read(&mut buffer).await;

            let request = String::from_utf8_lossy(&buffer);
            let (status_line, body) = if request.starts_with("GET /games") {
                let games = games.lock().await;
                let json = serde_json::to_string(&*games).expect("Failed to serialize games");
                (
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n",
                    json,
                )
            } else if request.starts_with("POST /games") {
                println!("Full request \n{}\n", request);
                let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
                println!("Incoming body {}\n", body);
                let new_game = serde_json::from_str(body);
                match new_game {
                    Ok(game) => {
                        games.lock().await.push(game);
                        ("HTTP/1.1 201 CREATED\r\n\r\n", "Game added!".to_string())
                    }
                    Err(e) => (
                        "HTTP/1.1 400 BAD REQUEST\r\n\r\n",
                        format!("Invalid game data. Error {}", e),
                    ),
                }
            } else {
                ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "Not Found".to_string())
            };

            let response = format!("{}{}", status_line, body);
            let _ = socket.write_all(response.as_bytes()).await;
        });
    }
}
