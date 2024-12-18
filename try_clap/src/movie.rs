pub struct Movie {
    pub name:String,
    pub year:u8,
    pub rating:f32
}

impl Movie {
    pub fn new(name:String,year:u8,rating:f32)->Self{
        Movie{
            name,
            year,
            rating
        }
    }
}