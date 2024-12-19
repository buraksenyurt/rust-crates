/*
    Örnek Komutlar

    cargo run -- help
    cargo run -- add -h
    cargo run -- remove -h

    cargo run -- add "The Matrix" 1995 9.4
    cargo run -- list
    cargo run -- remove "The Matrix"
    cargo run -- remove "Noname"
*/
use clap::{Parser, Subcommand};

use crate::{movie::Movie, movie_controller::MovieController};

#[derive(Parser)]
#[command(
    name = "My Favorite Movies",
    about = "Manage your favorite movies from the terminal"
)]
struct Terminal {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new movie
    Add {
        /// The title of the movie
        title: String,
        /// The release year of the movie
        year: u16,
        /// Your rating for the movie
        rating: f32,
    },
    /// List all movies
    List,
    /// Remove a movie by title
    Remove {
        /// The title of the movie to remove
        title: String,
    },
}

pub fn run() {
    let cli = Terminal::parse();
    let controller = MovieController;

    match cli.command {
        Commands::Add {
            title,
            year,
            rating,
        } => {
            let movie = Movie {
                name: title,
                year,
                rating,
            };
            if let Err(e) = controller.add(&movie) {
                eprintln!("Error adding movie: {}", e);
            } else {
                println!("Movie added successfully!");
            }
        }
        Commands::List => {
            if let Err(e) = controller.list() {
                eprintln!("Error listing movies: {}", e);
            }
        }
        Commands::Remove { title } => {
            if let Err(e) = controller.remove(title) {
                eprintln!("Error removing movie: {}", e);
            } else {
                println!("Movie removed successfully!");
            }
        }
    }
}
