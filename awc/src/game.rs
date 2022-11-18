use std::collections::HashMap;

use glam::uvec2;

use crate::{map::{self}, player::{self, Player, Team, Faction}, component::{self, EntityID, EntityType}, table::Table, tile, unit::{self}, ID};
use crate::component::*;

type Factory<T> = HashMap<ID, T>;

pub struct Game
{
    pub map : map::Map,
    components : component::Components,
    players : Table<player::ID, player::Player>,
    unit_factory : Factory<unit::Template>,
    tile_factory : Factory<tile::Template>
}

#[derive(Debug)]
pub enum Error{
    InvalidPosition,
    PlayerNotFound,
    TemplateNotFound
}

impl Game{
    pub fn new() -> Game{
        Game { 
            map: map::Map::new(uvec2(10, 10)), 
            players: Table::new(), 
            components : component::Components::new(), 
            unit_factory : Factory::new(),
            tile_factory : Factory::new()
        }
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
            self.components.insert(id, Component::Type(component::Type { type_id ,entity_type: EntityType::Tile}));
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

            if self.get_player(&owner).is_some(){
                
                let id = self.create_unit_from_template(type_id)?;
                self.components.insert(id, Component::Owner(Owner{owner}));
                self.components.insert(id, Component::Position(Position{pos}));
                self.map.add_unit(id);
                return Ok(id);
            }
            else{
                return Err(Error::PlayerNotFound);
            }
        }

        Err(Error::InvalidPosition)
    }

    fn create_unit_from_template(&mut self, type_id : unit::TypeID) -> Result<ID, Error>{

        if let Some(template) = self.unit_factory.get(&type_id).cloned(){
            let id = self.components.alloc_id();
            if !template.weapons.is_empty() {
                self.components.insert(id, Component::Armament(component::Armament::new(template.weapons)));
            }
            if let Some(movement) = template.movement{
                self.components.insert(id, Component::Movement(component::Movement::new(movement)));
            }
            
            self.components.insert(id, Component::Type(component::Type::new(type_id, EntityType::Unit)));
            self.components.insert(id, Component::Health(Health::default()));

            return Ok(id);
        }

        Err(Error::TemplateNotFound)
    }

    pub fn get_unit_in_pos(&self, target_pos : &map::Pos) -> Option<EntityID>{
        for unit in self.map.units(){
            let pos = self.components.get_position(unit).unwrap();
            if pos.pos == *target_pos{
                return Some(unit.clone())
            }
        }

        None
    }

    pub fn set_map_size(&mut self, size : map::Size){
        self.map.size = size;

        // TODO: Remove tiles/units out of range?
    }

    pub fn get_map_data(&self, alphabet : HashMap<tile::TypeID,char> ) -> map::Data{
        map::Data{alphabet, size : self.map.size, 
            
            tiles : self.map.tiles().map(|id| (
                self.components.positions.entry(id).unwrap().pos, 
                if let EntityType::Tile = self.components.types.entry(id).unwrap().entity_type{
                    self.components.types.entry(id).unwrap().type_id
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
            self.create_unit(unit.utype.type_id, pos, unit.owner.owner)?;
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

    pub fn add_unit_template(&mut self, id : ID, unit_template : unit::Template){
        self.unit_factory.insert(id, unit_template);
    }

    pub fn add_tile_template(&mut self, id : ID, tile_template : tile::Template){
        self.tile_factory.insert(id, tile_template);
    }

}

pub struct Turn
{
    pub turn : i32,
    pub player : player::ID
}