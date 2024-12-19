use std::fmt::Display;

pub struct Movie {
    pub name: String,
    pub year: u16,
    pub rating: f32,
}

impl Movie {
    pub fn new(name: String, year: u16, rating: f32) -> Self {
        Movie { name, year, rating }
    }
}

impl Display for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}). {} point", self.name, self.year, self.rating)
    }
}
