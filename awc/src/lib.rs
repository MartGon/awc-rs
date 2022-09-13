
mod unit;
mod tile;
mod movement;
mod table;

pub mod event;
pub mod component;
pub mod map;
pub mod weapon;
pub mod game;
pub mod player;

use crate::table::TableID;
use std::hash::Hash;

#[derive(Copy)]
pub struct ID(i32);

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

impl PartialEq for ID{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ID{
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for ID{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Clone for ID{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl ID{
    pub fn new(id : i32) -> ID{
        Self(id)
    }
}
