use std::{collections::{HashMap}, hash::Hash};

use crate::spritesheet::{Sprite, self};

pub enum RoadType{
    Vertical,
    Horizontal,
    BottomRight,
    BottomLeft,
    TopRight,
    TopLeft,
    HorizontalTop,
    HorizontalBottom,
    VerticalLeft,
    VerticalRight,
    VerticalHorizontal,
}

pub struct Road{
    sprites : HashMap<RoadType, Sprite>
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Default, Debug)]
pub struct Borders{
    pub top : bool,
    pub bottom : bool,
    pub left : bool,
    pub right : bool,
    pub top_left : bool,
    pub top_right : bool,
    pub bottom_left : bool,
    pub bottom_right : bool,
}

pub struct BorderedTile<S>{
    sprites : HashMap<Borders, S>
}

impl<S : spritesheet::Drawable + Clone> BorderedTile<S>{
    pub fn new( sprites: &[(Borders, S)]) -> BorderedTile<S>{
        BorderedTile { sprites: sprites.into_iter().cloned().collect() }
    }

    pub fn sprite(&mut self, border : &Borders) -> Option<&mut S>{
        self.sprites.get_mut(&border)
    }
}