use std::collections::HashMap;

use crate::{map, player};

pub struct Game
{
    pub map : map::Map,
    pub players : HashMap<player::ID, player::Player>,
    // pub components
    // pub entity factory
}

pub struct Turn
{
    pub turn : i32,
    pub player : player::ID
}