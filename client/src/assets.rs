use std::{fmt::{self}, path::Path, collections::HashMap, fs, default};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::tileset;

#[derive(Debug)]
pub enum Error{
    FileNotFound(std::io::Error, String),
    ParsingError(ron::error::SpannedError, String)
}   

impl fmt::Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::FileNotFound(e, s) => write!(f, "{}: {}", e, s),
            Self::ParsingError(e, s) => write!(f, "{}: {}", e, s),
        }
    }
}

pub type ErrorMap = HashMap<awc::tile::TypeID, self::Error>;

pub trait MasterFile<T: Default + Serialize + DeserializeOwned>{

    fn load_from_master_file<P: AsRef<Path> + Into<String> + Clone>(master_file : P) -> Result<(HashMap<awc::tile::TypeID, T>, ErrorMap), self::Error>{
        let tileset_str = fs::read_to_string(&master_file).map_err(|e| self::Error::FileNotFound(e, master_file.clone().into()));
        let tileset = ron::from_str::<HashMap<awc::tile::TypeID, String>>(&tileset_str.unwrap()).map_err(|e| self::Error::ParsingError(e, master_file.into())).unwrap();

        let mut error_map = ErrorMap::new();
        let tileset : HashMap<awc::ID, T> = tileset.into_iter().map(|(id, path)|{

            match fs::read_to_string(&path){
                Ok(file_data) => match ron::from_str::<T>(&file_data){
                    Ok(tile) => return (id, tile),
                    Err(e) => {error_map.insert(id, Error::ParsingError(e, path));},
                },
                Err(e) => {error_map.insert(id, self::Error::FileNotFound(e, path.into()));}
            }
    
            (id, T::default())
        }).collect();
        return Ok((tileset, error_map));
    }
}