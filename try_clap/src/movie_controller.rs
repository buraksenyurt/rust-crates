use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};

use crate::error::MovieError;
use crate::movie::Movie;

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

    pub fn remove(&self, movie_name: String) -> Result<(), MovieError> {
        let file = OpenOptions::new().read(true).open(FILE_NAME)?;
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();
        let mut found = false;

        for line in reader.lines() {
            let line = line?;
            if line.starts_with(&movie_name) {
                found = true;
                break;
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
