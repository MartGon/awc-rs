use std::{collections::{HashMap}, hash::Hash};
use awc::tile;
use macroquad::prelude::IVec2;
use serde::{Deserialize, Serialize};

use crate::spritesheet;

pub type Tileset = HashMap<awc::tile::TypeID, BorderedTile>;

// E.g. For water border tiles, either grass or mountain are valid.
#[derive(Clone, Hash, Debug, Deserialize, Serialize)]
pub enum BorderMaskEntry{
    Any,
    Some(Vec<tile::TypeID>)
}

impl BorderMaskEntry {
    
    pub fn some(ids : &[i32]) -> BorderMaskEntry{
        BorderMaskEntry::Some(ids.into_iter().map(|id| tile::TypeID::new(*id)).collect())
    }

    pub fn matches(&self, other : &tile::TypeID) -> bool{
        match self{
            BorderMaskEntry::Any => true,
            BorderMaskEntry::Some(l) => l.contains(other),
        }
    }
}

impl Default for BorderMaskEntry{
    fn default() -> Self {
        Self::Any
    }
}

pub const OFFSET_MIN : i32 = -1;
pub const OFFSET_MAX : i32 = 1;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct BordersMask{
    mask : HashMap<IVec2, BorderMaskEntry>,
}

impl BordersMask{
    pub fn new(borders : &[(IVec2, BorderMaskEntry)]) -> BordersMask{
        BordersMask { mask : borders.into_iter().cloned().collect() }
    }
    
    pub fn new_short( mask : BorderMaskEntry, offsets : &[IVec2]) -> BordersMask{
        BordersMask{ mask : offsets.into_iter().map(|o|(*o, mask.clone())).collect()}
    }

    pub fn matches(&self, borders : &Borders) -> bool{
        let res = true;
        for (offset, lb) in self.mask.iter(){
            if let Some(rb) = borders.get(offset){
                if !lb.matches(rb) {
                    // Found tile in target didn't match mask
                    return false;
                }
            }
            else{
                // Tile in border didn't contain mandatory tile in mask
                return false;
            }
        }

        res
    }
}

#[derive(Default)]
pub struct Borders{
    borders : HashMap<IVec2, tile::TypeID>,
}

impl Borders{
    pub fn get(&self, offset : &IVec2) -> Option<&tile::TypeID>{
        self.borders.get(offset)
    }

    pub fn insert(&mut self,  offset : IVec2, border : tile::TypeID){
        self.borders.insert(offset, border);
    }
}

#[derive(Serialize, Deserialize)]
pub struct BorderedTile{
    sprites : Vec<(BordersMask, spritesheet::SpriteType)>,
    default : spritesheet::SpriteType,
}

impl BorderedTile{

    pub fn new_short(default : spritesheet::SpriteType) -> BorderedTile{
        BorderedTile::new(default, &[])
    }

    pub fn new(default : spritesheet::SpriteType, sprites: &[(BordersMask, spritesheet::SpriteType)]) -> BorderedTile{
        BorderedTile { default,  sprites: sprites.into_iter().cloned().collect() }
    }

    pub fn sprite_mut(&mut self, border : &Borders) -> &mut spritesheet::SpriteType{
        for (b, sprite) in self.sprites.iter_mut(){   
            if b.matches(border){
                return sprite;
            }
        }
        &mut self.default
    }

    pub fn sprite(&self, border : &Borders) -> &spritesheet::SpriteType{
        for (b, sprite) in self.sprites.iter(){   
            if b.matches(border){
                return sprite;
            }
        }
        &self.default
    }
}