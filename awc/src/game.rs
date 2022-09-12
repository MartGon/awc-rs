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

    pub fn create_player(&mut self, team : TeamID) -> player::ID{
        let player_id = self.players.next_id();
        let player = Player::new(player_id, team);
        self.players.new_entry(player)
    }
}

pub struct Turn
{
    pub turn : i32,
    pub player : player::ID
}