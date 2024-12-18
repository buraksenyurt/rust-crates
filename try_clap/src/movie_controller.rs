const FILE_NAME: &str = "movies.csv";

pub struct MovieController;

impl MovieController{
    pub fn add(movie:&Movie) -> io::Result<()> {
        Ok(())
    }

    pub fn list() -> io::Result<()>{
        Ok(())
    }

    pub fn remove(movie_name:String)-> io::Result<()>  {
        Ok(())
    }
}