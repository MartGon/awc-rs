use std::{fmt, error};
use std::{slice::Iter, collections::HashMap};

use glam::{UVec2, uvec2};
use serde::de::Error;
use serde::{Serialize, Deserialize};

use crate::{component::{self, EntityID}, tile};


pub type Size = UVec2;
pub type Pos = UVec2;

pub struct Map
{
    tiles : Vec<component::EntityID>,
    pub size : Size,
}

#[derive(Debug)]
pub enum MapError{
    InvalidPosition
}

impl Map{
    pub fn new() -> Map{
        Map { tiles: Vec::new(), size: uvec2(10, 10) }
    }

    pub fn add_tile(&mut self, id : EntityID){
        self.tiles.push(id)
    }

    pub fn tiles(&self) -> Iter<EntityID>{
        self.tiles.iter()
    }

    pub fn is_pos_valid(&self, pos : Pos) -> bool{
        return pos.x < self.size.x && pos.y < self.size.y;
    }
}


pub struct Data
{
    pub alphabet : HashMap<tile::TypeID, char>,
    pub size : UVec2,
    pub tiles : HashMap<UVec2, tile::TypeID>,
}

#[derive(Serialize, Deserialize)]
struct FileData{
    pub alphabet : HashMap<tile::TypeID, char>,
    pub size : UVec2,
    pub tiles : Vec<String>,
}

impl Serialize for Data{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        
        let mut data = Vec::new();
        for y in 0..self.size.y{
            let mut line = String::new();
            for x in 0..self.size.x{
                let pos = uvec2(x, y);
                if let Some(tile) = self.tiles.get(&pos){
                    if let Some(entry) = self.alphabet.get(tile){
                        line.push(entry.clone());
                    }
                }
            }
            data.push(line);
        }

        let file_data = FileData{alphabet : self.alphabet.clone(), size : self.size, tiles : data};
        file_data.serialize(serializer)
    }
}

#[derive(Debug)]
pub enum DeserializeError{
    NotFoundInAlphabet(char)
}


impl error::Error for DeserializeError {}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DeserializeError::*;
        match self {
            NotFoundInAlphabet(c) => write!(f, "Could not find char '{}' in alphabet", c),
            // ...
        }
    }
}

impl<'de> Deserialize<'de> for Data{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let res = FileData::deserialize(deserializer);
        
        if let Ok(file_data) = res{
            let mut data = Data{alphabet : file_data.alphabet, size : file_data.size, tiles : HashMap::new()};

            for y  in 0..file_data.size.y{
                let line = &file_data.tiles[y as usize];
                for x in 0..file_data.size.x{
                    if let Some(c) = line.chars().nth(x as usize){
                        if let Some(tile_type) = data.alphabet.iter().find_map(|v| if *v.1 == c {Some(*v.0)} else {None}){
                            data.tiles.insert(uvec2(x, y), tile_type);
                        }
                        else{
                            return Err(DeserializeError::NotFoundInAlphabet(c)).map_err(D::Error::custom);
                        }
                    }
                }
            }

            Ok(data)
        }
        else{
            Err(res.err().unwrap())
        }
    }
}