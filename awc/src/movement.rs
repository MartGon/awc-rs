use std::{collections::HashMap};
use crate::{map, game};

use serde::{Serialize, Deserialize};

use crate::tile;

#[derive(Debug)]
pub enum Error{
    CouldNotReachTarget,
    EntityCannotMove
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
        };

        let success = |pos : &map::Pos| -> bool { *pos == dest };

        let paths = pathfinding::directed::dijkstra::dijkstra(&origin, successors, success);
        paths.filter(|p| p.1 <= self.range)
    }

    pub fn get_neighbors_pos(origin : map::Pos) -> Vec<map::Pos>{
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

