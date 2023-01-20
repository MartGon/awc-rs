use std::{collections::{HashMap, VecDeque}};

use glam::uvec2;

use crate::{map::{self}, player::{self, Player, Team, Faction}, component::{self, EntityID, EntityType}, table::Table, tile, unit::{self}, ID, movement, event::{Event, EventI}, command::{Command, CommandI, self}, turn::{Turn}};
use crate::component::*;

type Factory<T> = HashMap<ID, T>;

pub struct Game
{
    pub map : map::Map,
    components : component::Components,
    players : Table<player::ID, player::Player>,    
    current_turn : Option<Turn>,

    event_queue : VecDeque<Event>,
    event_history : Table<crate::ID, Event>,

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
            event_queue : VecDeque::new(),
            event_history : Table::new(),
            current_turn : None,

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


    // Tile

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

    // Units

    pub fn create_unit(&mut self, type_id : Option<unit::TypeID>, pos : map::Pos, owner : player::ID) -> Result<EntityID, Error>{

        if self.map.is_pos_valid(pos){

            if self.get_player(&owner).is_some(){
                
                let id;
                if let Some(type_id) = type_id{
                    id = self.create_unit_from_template(type_id)?;
                }
                else{
                    id = self.components.alloc_id();
                }
                self.components.insert(id, Component::Health(Health::default()));
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

    pub fn get_unit_in_pos(&self, target_pos : &map::Pos) -> Option<EntityID>{
        for unit in self.map.units(){
            let pos = self.components.get_position(unit).unwrap();
            if pos.pos == *target_pos{
                return Some(unit.clone())
            }
        }

        None
    }

    pub fn calc_path(&self, entity_id : ID, dest : map::Pos) -> Result<movement::Path, movement::Error> {
       movement::calc_path(&self, entity_id, dest)
    }

    pub fn calc_move_area(&self, entity_id : ID) -> Result<movement::MovementArea, movement::Error> {
        movement::calc_move_area(&self, entity_id)
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


    // Map data

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
        
        // Restore tiles
        for (pos, tile) in data.tiles{
            self.create_tile(tile, pos)?;
        }

        // Restore units
        for (pos, unit) in data.units{
            let id = self.create_unit(None, pos, unit.owner.owner)?;

            self.components.types.insert(id, unit.utype);
            if let Some(movement) = unit.movement{
                self.components.insert(id, component::Component::Movement(movement));
            }
            if let Some(armamnet) = unit.armament{
                self.components.insert(id, component::Component::Armament(armamnet));
            }
            
        }

        Ok(())
    }


    // Components

    pub fn insert_component(&mut self, entity : EntityID, component : component::Component){
        self.components.insert(entity, component)
    }

    pub fn components(&self) -> &component::Components{
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut component::Components{
        &mut self.components
    }


    // Turns

    pub fn get_turn(&self) -> Option<&Turn>{
        match &self.current_turn{
            Some(t) => Some(t),
            None => None,
        }
    }

    pub fn current_turn(&self) -> &Turn{
        self.current_turn.as_ref().expect("Game hasn't started")
    }

    pub(crate) fn get_turn_mut(&mut self)-> Option<&mut Turn>{
        match &mut self.current_turn{
            Some(t) => Some(t),
            None => None,
        }
    }

    pub(crate) fn end_turn(&mut self) {
        let next_player = self.find_next_turn_player();
        let current_day = self.current_turn().day;
        let next_day = if self.current_turn().player > next_player.id { current_day + 1 } else{ current_day };
        self.current_turn = Some(Turn::new(next_day, next_player.id));
    }

    fn find_next_player(&self, pid : player::ID) -> &Player{
        let next_pid = ID::new(pid.0 + 1);

        let next_player : &Player;
        if let Some(player) = self.players.get_entry(&next_pid){
            next_player = player;
        }
        else{
            next_player = self.players.get_entry(&ID::new(0)).expect("There are no players");
        }

        next_player
    }

    fn find_next_turn_player(&self) -> &Player{
        let pid = self.current_turn().player;
        let next_player = self.find_next_player(pid);

        if next_player.was_defeated{
            self.find_next_player(next_player.id)
        }else {
            next_player
        }
    }

    pub fn start(&mut self){
        self.current_turn = Some(Turn::new(0, 0.into()));
    }

    pub(crate) fn push_event(&mut self, event : Event){
        self.event_queue.push_back(event);
    }

    pub(crate) fn run_events(&mut self){
        while let Some(event) = self.event_queue.pop_front(){
            event.run(self);
        }
    }

    pub fn run_command(&mut self, command : Command, author : &player::ID) -> Result<(), command::Error>{
        let ret = command.execute(self, author);
        if let Ok(_) = ret{
            self.run_events();
        }

        ret
    }

    pub fn add_unit_template(&mut self, id : ID, unit_template : unit::Template){
        self.unit_factory.insert(id, unit_template);
    }

    pub fn add_tile_template(&mut self, id : ID, tile_template : tile::Template){
        self.tile_factory.insert(id, tile_template);
    }

}