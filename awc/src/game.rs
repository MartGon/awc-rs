use std::collections::HashMap;

use crate::{map::{self, Data, MapError}, player::{self, TeamID, Player}, component::{self, EntityID, EntityType}, table::Table, tile};
use crate::component::*;

pub struct Game
{
    pub map : map::Map,
    components : component::Components,
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

    pub fn create_tile(&mut self, type_id : tile::TypeID, pos : map::Pos) -> Result<EntityID, MapError>{
    
        if self.map.is_pos_valid(pos){
            let id = self.components.alloc_id();
            self.components.insert(id, Component::Type{0 : component::Type { entity_type: EntityType::Tile(type_id)}});
            self.components.insert(id, Component::Position(Position {pos}));
            self.map.add_tile(id);
            return Ok(id)
        }

        return Err(MapError::InvalidPosition);
    }

    pub fn get_tile_in_pos(&self, target_pos : &map::Pos) -> Option<EntityID>{
        for tile in self.map.tiles(){
            let pos = self.components.get_position(tile).unwrap();
            if pos.pos == *target_pos{
                return Some(tile.clone())
            }
        }

        None
    }

    pub fn set_map_size(&mut self, size : map::Size){
        self.map.size = size;

        // TODO: Remove tiles out of range?
    }

    pub fn get_map_data(&self, alphabet : HashMap<tile::TypeID,char> ) -> map::Data{
        map::Data{alphabet, size : self.map.size, tiles : self.map.tiles().map(|id| (
            self.components.positions.entry(id).unwrap().pos, 
            if let EntityType::Tile(tile_id) = self.components.types.entry(id).unwrap().entity_type{
                tile_id
            }
            else{
                panic!("WTF")
            })).collect()
        }
    }

    pub fn load_map_data(&mut self, data : map::Data) -> Result<(), MapError>{
        self.map = map::Map::new();
        for (pos, tile) in data.tiles{
            self.create_tile(tile, pos)?;
        }
        Ok(())
    }

    pub fn insert_component(&mut self, entity : EntityID, component : component::Component){
        self.components.insert(entity, component)
    }

    pub fn components(&self) -> &component::Components{
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut component::Components{
        &mut self.components
    }
}

pub struct Turn
{
    pub turn : i32,
    pub player : player::ID
}