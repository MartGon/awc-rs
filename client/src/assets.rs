use std::fmt;



#[derive(Debug)]
pub enum Error{
    FileNotFound(std::io::Error),
    ParsingError(ron::error::SpannedError)
}   

impl fmt::Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::FileNotFound(e) => e.fmt(f),
            Self::ParsingError(e) => e.fmt(f)
        }
    }
}

impl From<std::io::Error> for Error{
    fn from(e: std::io::Error) -> Self {
        Self::FileNotFound(e)
    }
}

impl From<ron::error::SpannedError> for Error{
    fn from(e: ron::error::SpannedError) -> Self {
        Self::ParsingError(e)
    } 
}