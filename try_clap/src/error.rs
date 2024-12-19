use std::fmt;

#[derive(Debug)]
pub enum MovieError {
    NotFound(String),
    IoError(std::io::Error),
}

impl fmt::Display for MovieError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MovieError::NotFound(name) => write!(f, "Movie not found: {}", name),
            MovieError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for MovieError {}

impl From<std::io::Error> for MovieError {
    fn from(err: std::io::Error) -> Self {
        MovieError::IoError(err)
    }
}
