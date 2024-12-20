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
use clap::{Parser, Subcommand, ValueEnum};
use std::str::FromStr;

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
    List {
        /// List by field
        field: Field,
        /// with ordering (Ascending / Descending)
        order: Order,
    },
    /// Remove a movie by title
    Remove {
        /// The title of the movie to remove
        title: String,
    },
}

#[derive(PartialEq, Clone, ValueEnum)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Clone, ValueEnum)]
pub enum Field {
    Name,
    Year,
    Rating,
}

impl FromStr for Field {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err("Field must be in format <field>:<order>".to_string());
        }

        match parts[1].to_lowercase().as_str() {
            "asc" => Order::Asc,
            "desc" => Order::Desc,
            _ => return Err("Order must be 'asc' or 'desc'".to_string()),
        };

        match parts[0].to_lowercase().as_str() {
            "name" => Ok(Field::Name),
            "year" => Ok(Field::Year),
            "rating" => Ok(Field::Rating),
            _ => Err("Field must be 'name', 'year', or 'rating'".to_string()),
        }
    }
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
        Commands::List { field, order } => {
            if let Err(e) = controller.list_by(field, order) {
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
