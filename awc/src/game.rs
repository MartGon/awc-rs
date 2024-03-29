use std::{collections::{HashMap, VecDeque}, fs};

use glam::uvec2;
use mlua::prelude::*;

use crate::{map::{self}, player::{self, Player, Team, Faction}, component::{self, EntityID, EntityType}, table::Table, tile, unit::{self}, ID, movement::{self, Path}, event::{Event, EventI, Notification, SubEvent, self}, command::{Command, CommandI, self}, turn::{Turn}, script::{self, Script}};
use crate::component::*;

type Factory<T> = HashMap<ID, T>;

pub struct GameState{
    pub map : map::Map,
    pub components : component::Components,
    pub players : Table<player::ID, player::Player>,    
    pub current_turn : Option<Turn>,
}

pub struct Game<'a: 'b, 'b>
{   
    pub game_state : GameState,

    // Move these to its own struct
    events : Table<ID, Event>,
    event_queue : VecDeque<ID>,
    event_history : Vec<ID>,

    unit_factory : Factory<unit::Template>,
    tile_factory : Factory<tile::Template>,

    lua_vm : &'a Lua,
    scripts : HashMap<String, Script<'b>>,
}

#[derive(Debug)]
pub enum Error{
    InvalidPosition,
    PlayerNotFound,
    TemplateNotFound,
    ScriptError,
}

impl<'a: 'b, 'b> Game<'a, 'b>{
    pub fn new(lua_vm : &'a Lua) -> Game<'a, 'b>{
        Game { 

            game_state : GameState{
                map: map::Map::new(uvec2(10, 10)), 
                players: Table::new(), 
                components : component::Components::new(), 
                current_turn : None
            },

            event_queue : VecDeque::new(),
            events : Table::new(),
            event_history : Vec::new(),

            unit_factory : Factory::new(),
            tile_factory : Factory::new(),
            
            lua_vm,
            scripts : HashMap::new(),
        }
    }

    // Players

    pub fn create_player(&mut self, team : Team, faction : Faction) -> player::ID{
        let player_id = self.game_state.players.next_id();
        let player = Player::new(player_id, team, faction);
        self.game_state.players.new_entry(player)
    }

    pub fn get_player_mut(&mut self, player_id : &player::ID) -> Option<&mut Player>{
        self.game_state.players.get_entry_mut(player_id)
    }

    pub fn get_player(&self, player_id : &player::ID) -> Option<&Player>{
        self.game_state.players.get_entry(player_id)
    }


    // Tile

    pub fn create_tile(&mut self, type_id : tile::TypeID, pos : map::Pos) -> Result<EntityID, Error>{
    
        if self.game_state.map.is_pos_valid(pos){
            let id = self.game_state.components.alloc_id();
            self.game_state.components.insert(id, Component::Type(component::Type { type_id ,entity_type: EntityType::Tile}));
            self.game_state.components.insert(id, Component::Position(Position {pos}));
            self.game_state.map.add_tile(id);
            return Ok(id)
        }

        return Err(Error::InvalidPosition);
    }

    pub fn get_tile_in_pos(&self, target_pos : &map::Pos) -> Option<EntityID>{
        for tile in self.game_state.map.tiles(){
            let pos = self.game_state.components.get_position(tile).unwrap();
            if pos.pos == *target_pos{
                return Some(tile.clone())
            }
        }

        None
    }

    // Units

    pub fn create_unit(&mut self, type_id : Option<unit::TypeID>, pos : map::Pos, owner : player::ID) -> Result<EntityID, Error>{

        if self.game_state.map.is_pos_valid(pos){

            if self.get_player(&owner).is_some(){
                
                let id;
                if let Some(type_id) = type_id{
                    id = self.create_unit_from_template(type_id)?;
                }
                else{
                    id = self.game_state.components.alloc_id();
                }
                self.game_state.components.insert(id, Component::Health(Health::default()));
                self.game_state.components.insert(id, Component::Owner(Owner{owner}));
                self.game_state.components.insert(id, Component::Position(Position{pos}));
                self.game_state.map.add_unit(id);
                return Ok(id);
            }
            else{
                return Err(Error::PlayerNotFound);
            }
        }

        Err(Error::InvalidPosition)
    }

    pub fn get_unit_in_pos(&self, target_pos : &map::Pos) -> Option<EntityID>{
        for unit in self.game_state.map.units(){
            let pos = self.game_state.components.get_position(unit).unwrap();
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
            let id = self.game_state.components.alloc_id();
            if !template.weapons.is_empty() {
                self.game_state.components.insert(id, Component::Armament(component::Armament::new(template.weapons)));
            }
            if let Some(movement) = template.movement{
                self.game_state.components.insert(id, Component::Movement(component::Movement::new(movement)));
            }
            
            self.game_state.components.insert(id, Component::Type(component::Type::new(type_id, EntityType::Unit)));
            self.game_state.components.insert(id, Component::Health(Health::default()));

            return Ok(id);
        }

        Err(Error::TemplateNotFound)
    }


    // Map data

    pub fn set_map_size(&mut self, size : map::Size){
        self.game_state.map.size = size;

        // TODO: Remove tiles/units out of range?
    }

    pub fn get_map_data(&self, alphabet : HashMap<tile::TypeID,char> ) -> map::Data{
        map::Data{alphabet, size : self.game_state.map.size, 
            
            tiles : self.game_state.map.tiles().map(|id| (
                self.game_state.components.positions.entry(id).unwrap().pos, 
                if let EntityType::Tile = self.game_state.components.types.entry(id).unwrap().entity_type{
                    self.game_state.components.types.entry(id).unwrap().type_id
                }
                else{
                    panic!("WTF")
                }
            )).collect(),
            
            units : self.game_state.map.units().map(|id|(
                self.game_state.components.positions.entry(id).unwrap().pos,
                unit::Unit{
                    utype : self.game_state.components.types.entry(id).unwrap().clone(),
                    position : self.game_state.components.positions.entry(id).unwrap().clone(),
                    health : self.game_state.components.healths.entry(id).unwrap().clone(),
                    owner : self.game_state.components.owners.entry(id).unwrap().clone(),
                    direction : if let Some(dir) = self.game_state.components.directions.entry(id) { Some(dir.clone()) } else { None},
                    armament : if let Some(armament) = self.game_state.components.armaments.entry(id) { Some(armament.clone() )} else { None },
                    movement : if let Some(movement) = self.game_state.components.movements.entry(id){ Some(movement.clone()) } else { None },
                    effects : if let Some(effects) = self.game_state.components.effectss.entry(id){ Some(effects.clone()) } else { None },
                }
            )).collect(),
        }
    }

    pub fn load_map_data(&mut self, data : map::Data) -> Result<(), Error>{

        // TODO: Remove old map's tiles components

        self.game_state.map = map::Map::new(data.size);
        
        // Restore tiles
        for (pos, tile) in data.tiles{
            self.create_tile(tile, pos)?;
        }

        // Restore units
        for (pos, unit) in data.units{
            let id = self.create_unit(None, pos, unit.owner.owner)?;

            self.game_state.components.types.insert(id, unit.utype);
            if let Some(movement) = unit.movement{
                self.game_state.components.insert(id, component::Component::Movement(movement));
            }
            if let Some(armamnet) = unit.armament{
                self.game_state.components.insert(id, component::Component::Armament(armamnet));
            }
            if let Some(effects) = unit.effects{
                self.game_state.components.insert(id, component::Component::Effects(effects));
            }
            
        }

        Ok(())
    }


    // Components

    pub fn insert_component(&mut self, entity : EntityID, component : component::Component){
        self.game_state.components.insert(entity, component)
    }

    pub fn components(&self) -> &component::Components{
        &self.game_state.components
    }

    pub fn components_mut(&mut self) -> &mut component::Components{
        &mut self.game_state.components
    }


    // Turns

    pub fn get_turn(&self) -> Option<&Turn>{
        match &self.game_state.current_turn{
            Some(t) => Some(t),
            None => None,
        }
    }

    pub fn current_turn(&self) -> &Turn{
        self.game_state.current_turn.as_ref().expect("Game hasn't started")
    }

    pub(crate) fn get_turn_mut(&mut self)-> Option<&mut Turn>{
        match &mut self.game_state.current_turn{
            Some(t) => Some(t),
            None => None,
        }
    }

    pub(crate) fn end_turn(&mut self) {
        let next_player = self.find_next_turn_player();
        let current_day = self.current_turn().day;
        let next_day = if self.current_turn().player > next_player.id { current_day + 1 } else{ current_day };
        self.game_state.current_turn = Some(Turn::new(next_day, next_player.id));
    }

    fn find_next_player(&self, pid : player::ID) -> &Player{
        let next_pid = ID::new(pid.0 + 1);

        let next_player : &Player;
        if let Some(player) = self.game_state.players.get_entry(&next_pid){
            next_player = player;
        }
        else{
            next_player = self.game_state.players.get_entry(&ID::new(0)).expect("There are no players");
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
        self.game_state.current_turn = Some(Turn::new(0, 0.into()));

        self.init_scripts().expect("Failed to init scripts");
    }

    // Events / Commands

    pub(crate) fn push_event(&mut self, event : Event) -> ID{
        let id = self.events.new_entry(event);
        self.event_queue.push_back(id);

        id
    }

    pub(crate) fn run_events(&mut self){
        while let Some(event_id) = self.event_queue.front().cloned(){
            let event = self.events.get_entry(&event_id).expect("Could not find event by event id").clone();

            // Notify PRE
            self.notifiy_event(&event, Notification::Pre);

            if let Some(post_event_id) = self.event_queue.front().cloned(){
                if post_event_id == event_id{
                    event.run(self);
                    self.event_queue.pop_front();
                    self.event_history.push(event_id);

                    // Notify Post
                }
            }
        }
    }

    fn notifiy_event(&mut self, event : &Event, not_type : Notification){
        let effects = &self.game_state.components.effectss;
        let effects : HashMap<ID, Effects> = effects.into_iter().map(|a| (a.0.clone(), a.1.clone())).collect();
        for (id, effects) in effects{
            for e in &effects.effects{
                if let Some(script) = self.scripts.get(&e.script){
                    e.notify(&mut self.game_state, script, not_type.clone(), event);    
                }    
            }
        }
    }

    pub fn run_command(&mut self, command : Command, author : &player::ID) -> Result<(), command::Error>{
        let ret = command.execute(self, author);
        if let Ok(_) = ret{
            self.run_events();
        }

        ret
    }

    // Templates

    pub fn add_unit_template(&mut self, id : ID, unit_template : unit::Template){
        self.unit_factory.insert(id, unit_template);
    }

    pub fn add_tile_template(&mut self, id : ID, tile_template : tile::Template){
        self.tile_factory.insert(id, tile_template);
    }

    // Scripts

    pub fn load_script<S: Into<String> + Clone, P: AsRef<std::path::Path> + Into<String> + Clone>(&mut self, name : &S, script_file : P) -> Result<(), Error>{

        let script = Script::from_file(self.lua_vm, name.clone().into(), script_file).expect("Error on loading script");
        self.scripts.insert(name.clone().into(), script);

        Ok(())
    }

    fn init_scripts(&mut self) -> Result<(), Error>{

        let lua = self.lua_vm;
        let my_sum = self.lua_vm.create_function(|l, (a, b) : (i32, i32)|{

            Ok(a + b)
        }).expect("Function code is wrong");
        lua.globals().set("my_sum", my_sum).expect("Error setting function");

        Ok(())
    }

}

impl LuaUserData for &mut GameState{
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(_fields: &mut F) {
        
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {

        methods.add_method_mut("end_turn", |_, game_state, ()|{
            let size = game_state.map.size;
            game_state.current_turn.as_mut().unwrap().day = 10;
            println!("Map size is {}", size);
            Ok(())
        });

        methods.add_method("print_map_size", |_, game_state, ()|{
            let size = game_state.map.size;
            println!("Map size is {}", size);
            Ok(())
        });

    }
}