use std::{fmt, path::Path, collections::HashMap, fs, hash::Hash};

use serde::{Serialize, de::DeserializeOwned};

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

pub type ErrorMap<ID> = HashMap<ID, self::Error>;

pub trait MasterFile<T: Serialize + DeserializeOwned>{

    fn load_from_master_file<ID: Hash + Copy + Eq + DeserializeOwned, P: AsRef<Path> + Into<String> + Clone>(master_file : P, default : T) -> Result<(HashMap<ID, T>, ErrorMap<ID>), self::Error> where T: Clone{
       load_from_master_file::<T, ID, P, _>(master_file, || default.clone())
    }

    fn load_from_master_file_default<
        ID: Hash + Copy + Eq + DeserializeOwned, 
        P: AsRef<Path> + Into<String> + Clone>
        (master_file : P) -> Result<(HashMap<ID, T>, ErrorMap<ID>), self::Error> 
        where T: Default
    {
            
        load_from_master_file_default(master_file)
    }
}

fn load_from_master_file<T: Serialize + DeserializeOwned, ID: Hash + Copy + Eq + DeserializeOwned, P: AsRef<Path> + Into<String> + Clone, F: Fn() -> T>(master_file : P, default : F) -> Result<(HashMap<ID, T>, ErrorMap<ID>), self::Error>{
    let tileset_str = fs::read_to_string(&master_file).map_err(|e| self::Error::FileNotFound(e, master_file.clone().into()));
    let tileset = ron::from_str::<HashMap<ID, String>>(&tileset_str.unwrap()).map_err(|e| self::Error::ParsingError(e, master_file.into())).unwrap();

    let mut error_map = ErrorMap::new();
    let tileset : HashMap<ID, T> = tileset.into_iter().map(|(id, path)|{

        match fs::read_to_string(&path){
            Ok(file_data) => match ron::from_str::<T>(&file_data){
                Ok(tile) => return (id, tile),
                Err(e) => {error_map.insert(id, Error::ParsingError(e, path));},
            },
            Err(e) => {error_map.insert(id, self::Error::FileNotFound(e, path.into()));}
        }

        (id, default())
    }).collect();
    return Ok((tileset, error_map));
}

fn load_from_master_file_default<T: Serialize + DeserializeOwned + Default, ID: Hash + Copy + Eq + DeserializeOwned, P: AsRef<Path> + Into<String> + Clone>(master_file : P) -> Result<(HashMap<ID, T>, ErrorMap<ID>), self::Error>{
    load_from_master_file(master_file,  T::default)
}