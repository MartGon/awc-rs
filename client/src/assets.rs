use std::fmt::{self};

#[derive(Debug)]
pub enum Error{
    FileNotFound(std::io::Error, String),
    ParsingError(ron::error::SpannedError)
}   

impl fmt::Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::FileNotFound(e, s) => write!(f, "{}: {}", e, s),
            Self::ParsingError(e) => e.fmt(f)
        }
    }
}

impl From<ron::error::SpannedError> for Error{
    fn from(e: ron::error::SpannedError) -> Self {
        Self::ParsingError(e)
    } 
}