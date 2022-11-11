use std::collections::HashMap;

use glam::uvec2;

use crate::{map::{self}, player::{self, Player, Team, Faction}, component::{self, EntityID, EntityType}, table::Table, tile, unit, movement};
use crate::component::*;

pub struct Game
{
    pub map : map::Map,
    components : component::Components,
    players : Table<player::ID, player::Player>,
    // pub entity factory
}

#[derive(Debug)]
pub enum Error{
    InvalidPosition,
    PlayerNotFound
}

impl Game{
    pub fn new() -> Game{
        Game { map: map::Map::new(uvec2(10, 10)), players: Table::new(), components : component::Components::new() }
    }

    pub fn create_player(&mut self, team : Team, faction : Faction) -> player::ID{
        let player_id = self.players.next_id();
        let player = Player::new(player_id, team, faction);
        self.players.new_entry(player)
    }

    pub fn get_player_mut(&mut self, player_id : &player::ID) -> Option<&mut Player>{
        self.players.get_entry_mut(player_id)
    }

    pub fn get_player(&self, player_id : &player::ID) -> Option<&Player>{
        self.players.get_entry(player_id)
    }

    pub fn create_tile(&mut self, type_id : tile::TypeID, pos : map::Pos) -> Result<EntityID, Error>{
    
        if self.map.is_pos_valid(pos){
            let id = self.components.alloc_id();
            self.components.insert(id, Component::Type(component::Type { entity_type: EntityType::Tile(type_id)}));
            self.components.insert(id, Component::Position(Position {pos}));
            self.map.add_tile(id);
            return Ok(id)
        }

        return Err(Error::InvalidPosition);
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

    pub fn create_unit(&mut self, type_id : unit::TypeID, pos : map::Pos, owner : player::ID) -> Result<EntityID, Error>{

        if self.map.is_pos_valid(pos){
            let id = self.components.alloc_id();
            self.components.insert(id, Component::Type(component::Type{entity_type : EntityType::Unit(type_id)}));
            self.components.insert(id, Component::Position(Position{pos}));
            self.components.insert(id, Component::Health(Health::default()));

            if self.get_player(&owner).is_some(){
                self.components.insert(id, Component::Owner(Owner{owner}));
                self.map.add_unit(id);
                return Ok(id);
            }
            else{
                return Err(Error::PlayerNotFound);
            }
        }

        Err(Error::InvalidPosition)
    }

    pub fn set_map_size(&mut self, size : map::Size){
        self.map.size = size;

        // TODO: Remove tiles out of range?
    }

    pub fn get_map_data(&self, alphabet : HashMap<tile::TypeID,char> ) -> map::Data{
        map::Data{alphabet, size : self.map.size, 
            
            tiles : self.map.tiles().map(|id| (
                self.components.positions.entry(id).unwrap().pos, 
                if let EntityType::Tile(tile_id) = self.components.types.entry(id).unwrap().entity_type{
                    tile_id
                }
                else{
                    panic!("WTF")
                }
            )).collect(),
            
            units : self.map.units().map(|id|(
                self.components.positions.entry(id).unwrap().pos,
                unit::Unit{
                    utype : self.components.types.entry(id).unwrap().clone(),
                    position : self.components.positions.entry(id).unwrap().clone(),
                    health : self.components.healths.entry(id).unwrap().clone(),
                    owner : self.components.owners.entry(id).unwrap().clone(),
                    direction : if let Some(dir) = self.components.directions.entry(id) { Some(dir.clone()) } else { None},
                    armament : if let Some(armament) = self.components.armaments.entry(id) { Some(armament.clone() )} else { None },
                    movement : if let Some(movement) = self.components.movements.entry(id){ Some(movement.clone()) } else { None },
                }
            )).collect(),
        }
    }

    pub fn load_map_data(&mut self, data : map::Data) -> Result<(), Error>{

        // TODO: Remove old map's tiles components

        self.map = map::Map::new(data.size);
        
        for (pos, tile) in data.tiles{
            self.create_tile(tile, pos)?;
        }

        for (pos, unit) in data.units{
            println!("For each unit");
            self.create_unit(unit.utype.entity_type.unit_type(), pos, unit.owner.owner).expect("Error on creating unit from map data");
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