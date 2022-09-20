use std::{collections::{HashMap}, hash::Hash};
use awc::tile;

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

// TODO: This could be changed to a HashMap<IVec2 (offset), Border>
#[derive(Hash, PartialEq, Eq, Clone, Default, Debug)]
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