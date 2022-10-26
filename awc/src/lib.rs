
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

use serde::{Deserialize, Serialize};

use crate::table::TableID;
use std::{hash::Hash, borrow::Borrow};

#[derive(Copy, Hash, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ID(pub u32);

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

impl From<u32> for ID{
    fn from(a: u32) -> Self {
        ID::new(a)
    }
}

impl AsRef<u32> for ID{
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl Borrow<u32> for ID{
    fn borrow(&self) -> &u32 {
        &self.0
    }
}

impl ID{
    pub fn new(id : u32) -> ID{
        Self(id)
    }
}
