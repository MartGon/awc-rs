use std::{collections::{HashMap}, hash::Hash};
use awc::tile;
use macroquad::prelude::IVec2;

use crate::spritesheet;

// Some could hold a list of valid tile::TypeID.
// E.g. For water border tiles, either grass or mountain are valid.
#[derive(Clone, Hash, Debug, Eq)]
pub enum Border{
    Any,
    Some(Vec<tile::TypeID>)
}

impl Default for Border{
    fn default() -> Self {
        Self::Any
    }
}

// It's not commutative. Left side acts as mask. Right side as target
impl PartialEq for Border{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Some(l0), Self::Some(r0)) => l0.contains(r0.get(0).unwrap()),
            (Self::Some(_l0), Self::Any) => false,
            (Self::Any, Self::Some(_r0)) => true,
            (Self::Any, Self::Any) => true,
        }
    }
}

pub const OFFSET_MIN : i32 = -1;
pub const OFFSET_MAX : i32 = 1;

#[derive(Eq, Clone, Default, Debug)]
pub struct Borders{
    borders : HashMap<IVec2, Border>,
}

impl Borders{
    pub fn new(borders : &[(IVec2, Border)]) -> Borders{
        Borders { borders : borders.into_iter().cloned().collect() }
    }

    pub fn get_mut(&mut self, offset : &IVec2) -> Option<&mut Border>{
        self.borders.get_mut(offset)
    }

    pub fn insert(&mut self,  offset : IVec2, border : Border){
        self.borders.insert(offset, border);
    }
}

impl PartialEq for Borders{
    fn eq(&self, other: &Self) -> bool{
        let res = true;
        for (offset, lb) in self.borders.iter(){
            if let Some(rb) = other.borders.get(offset){
                if lb != rb {
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

pub struct BorderedTile<S>{
    sprites : Vec<(Borders, S)>,
    default : S,
}

impl<S : spritesheet::Drawable + Clone> BorderedTile<S>{
    pub fn new(default : S, sprites: &[(Borders, S)]) -> BorderedTile<S>{
        BorderedTile { default,  sprites: sprites.into_iter().cloned().collect() }
    }

    pub fn sprite(&mut self, border : &Borders) -> &mut S{
        for (b, sprite) in self.sprites.iter_mut(){   
            if b == border{
                return sprite;
            }
        }
        &mut self.default
    }
}