use std::{iter::Iterator, slice::Iter};

use crate::component::{self, EntityID};

pub struct Size
{
    pub width : i32,
    pub height : i32
}

#[derive(PartialEq, Eq, Debug)]
pub struct Pos
{
    pub x : i32,
    pub y : i32,
    pub z : i32,
}

impl Pos{
    pub fn new(x : i32, y : i32, z : i32) -> Pos{
        Pos{x, y, z}
    }
}

pub struct Map
{
    tiles : Vec<component::EntityID>,
    pub size : Size,
}

impl Map{
    pub fn new() -> Map{
        Map { tiles: Vec::new(), size: Size { width: 10, height: 10 } }
    }

    pub fn add_tile(&mut self, id : EntityID){
        self.tiles.push(id)
    }

    pub fn tiles(&self) -> Iter<EntityID>{
        self.tiles.iter()
    }
}