use std::env;

use crate::{movie::Movie, movie_controller::MovieController};

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let controller = MovieController;

    if args.len() < 2 {
        println!("No command provided. Use 'help' for usage details.");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 5 {
                println!("Usage: add <movie name> <year> <rating>");
                println!("Example: add 'The Matrix' 1999 9.5");
                return;
            }
            let movie = Movie {
                name: args[2].clone(),
                year: args[3]
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid year: {}", args[3])),
                rating: args[4]
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid rating: {}", args[4])),
            };
            if let Err(e) = controller.add(&movie) {
                println!("Failed to add movie: {}", e);
            } else {
                println!("Movie added successfully: {}", movie);
            }
        }
        "list" => {
            if let Err(e) = controller.list() {
                println!("Failed to list movies: {}", e);
            }
        }
        "remove" => {
            if args.len() < 3 {
                println!("Usage: remove <movie name>");
                println!("Example: remove 'The Matrix'");
                return;
            }
            if let Err(e) = controller.remove(args[2].clone()) {
                println!("Failed to remove movie: {}", e);
            } else {
                println!("Movie removed successfully: {}", args[2]);
            }
        }
        "help" => {
            println!(
                r#"
        Movie Manager CLI
        
        Available commands:
          add <movie name> <year> <rating>   Add a new movie
            Example: add "The Matrix" 1999 9.5
        
          list                               List all movies
        
          remove <movie name>                Remove a movie by name
            Example: remove "The Matrix"
        
          help                               Show this help message
        "#
            );
        }
        _ => {
            println!("Invalid command. Use 'help' for usage details.");
        }
    }
}
