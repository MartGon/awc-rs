use std::collections::HashMap;
use crate::{map, player::{self, TeamID, Player}, component, table::Table};

pub struct Game
{
    pub map : map::Map,
    pub players : Table<player::ID, player::Player>,
    pub components : component::Components,
    // pub entity factory
}

impl Game{
    pub fn new() -> Game{
        Game { map: map::Map::new(), players: Table::new(), components : component::Components::new() }
    }

    pub fn create_player(self, team : TeamID) -> player::ID{
        Player::new(players, teamId)
    }
}

pub struct Turn
{
    pub turn : i32,
    pub player : player::ID
}