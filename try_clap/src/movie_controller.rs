use std::io;

use crate::movie::Movie;

const FILE_NAME: &str = "movies.csv";

pub struct MovieController;

impl MovieController {
    pub fn add(&self, movie: &Movie) -> io::Result<()> {
        Ok(())
    }

    pub fn list(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn remove(&self, movie_name: String) -> io::Result<()> {
        Ok(())
    }
}
