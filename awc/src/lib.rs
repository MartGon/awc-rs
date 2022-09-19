
mod unit;
mod movement;
mod table;

pub mod event;
pub mod component;
pub mod map;
pub mod weapon;
pub mod game;
pub mod player;
pub mod tile;

use crate::table::TableID;
use std::hash::Hash;

#[derive(Copy, Hash, Debug, PartialEq, Eq, Clone)]
pub struct ID(pub i32);

impl TableID for ID{
    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl Default for ID{
    fn default() -> Self {
        ID::new(0)
    }
}

impl ID{
    pub fn new(id : i32) -> ID{
        Self(id)
    }
}
