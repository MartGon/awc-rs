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

    fn load_from_master_file<ID: Hash + Clone + Eq + DeserializeOwned, P: AsRef<Path> + Into<String> + Clone>(master_file : P, default : T) -> Result<(HashMap<ID, T>, ErrorMap<ID>), self::Error> where T: Clone{
       load_from_master_file::<T, ID, P, _>(master_file, || default.clone())
    }

    fn load_from_master_file_default<
        ID: Hash + Eq + Clone + DeserializeOwned, 
        P: AsRef<Path> + Into<String> + Clone>
        (master_file : P) -> Result<(HashMap<ID, T>, ErrorMap<ID>), self::Error> 
        where T: Default
    {
            
        load_from_master_file_default(master_file)
    }
}

impl<T: Serialize + DeserializeOwned> MasterFile<T> for T{

}

fn load_from_master_file<T: Serialize + DeserializeOwned, ID: Hash + Clone + Eq + DeserializeOwned, P: AsRef<Path> + Into<String> + Clone, F: Fn() -> T>(master_file : P, default : F) -> Result<(HashMap<ID, T>, ErrorMap<ID>), self::Error>{
    
    let masterfile = load_master_file::<ID, P>(master_file)?;
    let mut error_map = ErrorMap::new();
    let tileset : HashMap<ID, T> = masterfile.into_iter().map(|(id, path)|{

        match fs::read_to_string(&path){
            Ok(file_data) => match ron::from_str::<T>(&file_data){
                Ok(tile) => return (id.clone(), tile),
                Err(e) => {error_map.insert(id.clone(), Error::ParsingError(e, path));},
            },
            Err(e) => {error_map.insert(id.clone(), self::Error::FileNotFound(e, path.into()));}
        }

        (id, default())
    }).collect();
    return Ok((tileset, error_map));
}

pub fn load_master_file<ID: Hash + Clone + Eq + DeserializeOwned, P: AsRef<Path> + Into<String> + Clone>(master_file : P) -> Result<HashMap<ID, String>, self::Error>{
    let masterfile_str = fs::read_to_string(&master_file).map_err(|e| self::Error::FileNotFound(e, master_file.clone().into()));
    ron::from_str::<HashMap<ID, String>>(&masterfile_str.unwrap()).map_err(|e| self::Error::ParsingError(e, master_file.into()))
}

fn load_from_master_file_default<T: Serialize + DeserializeOwned + Default, ID: Hash + Clone + Eq + DeserializeOwned, P: AsRef<Path> + Into<String> + Clone>(master_file : P) -> Result<(HashMap<ID, T>, ErrorMap<ID>), self::Error>{
    load_from_master_file(master_file,  T::default)
}