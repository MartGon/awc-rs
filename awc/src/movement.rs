use std::{collections::HashMap};
use crate::{map, game, ID};

use serde::{Serialize, Deserialize};

use crate::tile;

#[derive(Debug)]
pub enum Error{
    CouldNotReachTarget,
    DestinationSameAsOrigin,
    EntityCannotMove,
}

pub type MoveCost = u32;
pub type Path = (Vec<map::Pos>, MoveCost);

#[derive(Serialize, Deserialize, Clone)]
pub struct Movement
{
    range : u32,
    traverse_cost : HashMap<tile::TypeID, MoveCost>
}

impl Movement{

    pub fn new(range : u32, traverse_cost : &[(tile::TypeID, MoveCost)]) -> Movement{
        Movement { range, traverse_cost: traverse_cost.into_iter().cloned().collect() }
    }

    pub fn get_move_cost(&self, tile : tile::TypeID) -> Option<MoveCost>{
        if let Some(traverse_cost) = self.traverse_cost.get(&tile){
            Some(*traverse_cost)
        }
        else{
            None
        }
    }

    pub fn get_path(&self, game : &game::Game, origin : map::Pos, dest : map::Pos) -> Option<Path>{

        let successors = |origin : &map::Pos| -> Vec<(map::Pos, MoveCost)>{
            self.successors(game, origin)
        };

        let success = |pos : &map::Pos| -> bool { *pos == dest };
        let paths = pathfinding::directed::dijkstra::dijkstra(&origin, successors, success);
        paths.filter(|p| p.1 <= self.range)
    }

    pub fn get_area(&self, game : &game::Game, origin : &map::Pos) -> MovementArea{
        
        let successors = |origin : &map::Pos| -> Vec<(map::Pos, MoveCost)>{
            self.successors(game, origin)
        };

        let distance = |a : &map::Pos, b : &map::Pos| -> u32 {let diff = a.as_ivec2() - b.as_ivec2(); (diff.x.abs() + diff.y.abs()) as u32};
        let stop = | pos : &map::Pos| -> bool { distance(origin, pos) > self.range};
        let area = pathfinding::directed::dijkstra::dijkstra_partial(origin, successors, stop);
        MovementArea::new(*origin, self.range, area.0)
    }

    fn successors(&self, game: &game::Game, origin : &map::Pos) -> Vec<(map::Pos, MoveCost)>{
        let neighs = Movement::get_neighbors_pos(*origin);
        neighs.into_iter().filter_map(|p|{
            if game.map.is_pos_valid(p) {
                if let Some(tile_id) = game.get_tile_in_pos(&p){
                    let tile_type = game.components().get_type(&tile_id).expect("Movement: Tile didn't have a type");
                    if let Some(move_cost) = self.get_move_cost(tile_type.type_id){
                        return Some((p, move_cost));
                    }
                }
            }

            None
        }).collect()
    }

    fn get_neighbors_pos(origin : map::Pos) -> Vec<map::Pos>{
        let offsets = [map::Offset::new(-1, 0), map::Offset::new(1, 0), map::Offset::new(0, -1), map::Offset::new(0, 1)];
        let mut neighs = Vec::with_capacity(4);
        for o in offsets{
            if let Some(neigh) = map::add_offset(origin, o){
                neighs.push(neigh);
            }
        }

        neighs
    }
}


#[derive(new)]
pub struct MovementArea{
    origin : map::Pos,
    range : u32,
    area : HashMap<map::Pos, (map::Pos, MoveCost)>
}

impl MovementArea{

    pub fn get_origin(&self) -> map::Pos{
        self.origin
    }

    pub fn get_tiles(&self) -> Vec<map::Pos>{
        self.area.iter().filter_map(|t| if t.1.1 <= self.range {Some(t.0.clone())} else {None} ).collect()
    }

    pub fn is_in_area(&self, pos : &map::Pos) -> bool{
        self.area.contains_key(pos)
    }

    pub fn get_cost(&self, pos : &map::Pos) -> Option<MoveCost>{

        if let Some((_, cost)) = self.area.get(pos){
            return Some(*cost);
        }

        None
    }

    pub fn get_path(&self, dest : &map::Pos) -> Option<Path>{
        if let Some(cost) = self.get_cost(dest){
            return Some((pathfinding::directed::dijkstra::build_path(&dest, &self.area), cost));
        }
        
        None
    }
}


pub fn calc_path(game : &game::Game, entity_id : ID, dest : map::Pos) -> Result<Path, Error>{
        
    let pos = game.components().get_position(&entity_id).expect("Entity didn't have a position");
    if let Some(movement) = game.components().get_movement(&entity_id){
        let movement = &movement.movement;
        if let Some(path) = movement.get_path(&game, pos.pos, dest){
            return Ok(path);
        }

        return Err(Error::CouldNotReachTarget);
    }

    Err(Error::EntityCannotMove)
}

pub fn calc_move_area(game : &game::Game, entity_id : ID) -> Result<MovementArea, Error>{

    let pos = game.components().get_position(&entity_id).expect("Entity didn't have a position");
    if let Some(movement) = game.components().get_movement(&entity_id){
        let movement = &movement.movement;
        return Ok(movement.get_area(&game, &pos.pos));
    }

    Err(Error::EntityCannotMove)
}
