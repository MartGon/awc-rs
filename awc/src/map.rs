use std::{slice::Iter, collections::HashMap};

use glam::{UVec2, uvec2};
use serde::{Serialize, Deserialize, ser::SerializeStruct};

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

impl Serialize for Data{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("Data", 2)?;
        state.serialize_field("alphabet", &self.alphabet)?;
        state.serialize_field("size", &self.size)?;
        
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
        state.serialize_field("tiles", &data)?;

        state.end()
    }
}