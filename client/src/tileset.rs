use std::{collections::{HashMap}, hash::Hash};
use awc::tile;

use crate::spritesheet;

// This could be changed to a HashMap<IVec2 (offset), TileID>
// It could also be renamed to neighbours or something along those lines
// Add neighbour enum (None, Any, Some)

#[derive(Clone, Copy, Hash, Debug, Eq)]
pub enum Border{
    Any,
    Some(tile::TypeID)
}

impl Default for Border{
    fn default() -> Self {
        Self::Any
    }
}

impl PartialEq for Border{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Some(l0), Self::Some(r0)) => l0 == r0,
            (Self::Some(_l0), Self::Any) => false,
            (Self::Any, Self::Some(_r0)) => true,
            (Self::Any, Self::Any) => true,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Default, Debug)]
pub struct Borders{
    pub top : Border,
    pub bottom : Border,
    pub left : Border,
    pub right : Border,
    pub top_left : Border,
    pub top_right : Border,
    pub bottom_left : Border,
    pub bottom_right : Border,
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