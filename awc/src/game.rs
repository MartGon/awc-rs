use crate::{map, player::{self, TeamID, Player}, component::{self, EntityID, EntityType, Component, Position}, table::Table, tile};

pub struct Game
{
    pub map : map::Map,
    pub components : component::Components,
    players : Table<player::ID, player::Player>,
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

    pub fn get_player(&mut self, player_id : &player::ID) -> Option<&mut Player>{
        self.players.get_entry(player_id)
    }

    pub fn create_tile(&mut self) -> EntityID{
        let id = self.components.alloc_id();
        self.components.insert(id, component::Component::Type{0 : component::Type { entity_type: EntityType::Tile(tile::TypeID::new(0))}});
        id
    }
}

pub struct Turn
{
    pub turn : i32,
    pub player : player::ID
}