use crate::equiped::{Field, Order};
use crate::error::MovieError;
use crate::movie::Movie;
use std::cmp::Ordering;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};

const FILE_NAME: &str = "movies.csv";

pub struct MovieController;

impl MovieController {
    pub fn add(&self, movie: &Movie) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(FILE_NAME)?;
        writeln!(file, "{},{},{}", movie.name, movie.year, movie.rating)?;
        Ok(())
    }

    pub fn list(&self) -> io::Result<()> {
        let file = OpenOptions::new().read(true).open(FILE_NAME)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() == 3 {
                println!("{}, {}. ({})", fields[0], fields[1], fields[2]);
            }
        }

        Ok(())
    }

    pub fn list_by(&self, field: Field, order: Order) -> io::Result<()> {
        let file = OpenOptions::new().read(true).open(FILE_NAME)?;
        let reader = BufReader::new(file);
        let mut movies = vec![];
        for line in reader.lines() {
            let line = line?;
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() == 3 {
                movies.push(Movie::new(
                    fields[0].parse().unwrap(),
                    fields[1].parse().unwrap(),
                    fields[2].parse().unwrap(),
                ));
            }
        }

        match field {
            Field::Name => {
                if order == Order::Asc {
                    movies.sort_by(|a, b| a.name.cmp(&b.name));
                } else {
                    movies.sort_by(|a, b| b.name.cmp(&a.name));
                }
            }
            Field::Year => {
                if order == Order::Asc {
                    movies.sort_by(|a, b| a.year.cmp(&b.year));
                } else {
                    movies.sort_by(|a, b| b.year.cmp(&a.year));
                }
            }
            Field::Rating => {
                if order == Order::Asc {
                    movies
                        .sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(Ordering::Equal));
                } else {
                    movies
                        .sort_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap_or(Ordering::Equal));
                }
            }
        }

        for movie in movies {
            println!("{}", movie);
        }

        Ok(())
    }

    pub fn remove(&self, movie_name: String) -> Result<(), MovieError> {
        let file = OpenOptions::new().read(true).open(FILE_NAME)?;
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();
        let mut found = false;

        for line in reader.lines() {
            let line = line?;
            if line.starts_with(&movie_name) {
                found = true;
                continue;
            }
            lines.push(line);
        }

        if found {
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(FILE_NAME)?;
            for line in lines {
                writeln!(file, "{}", line)?;
            }
        } else {
            return Err(MovieError::NotFound(movie_name));
        }

        Ok(())
    }
}
