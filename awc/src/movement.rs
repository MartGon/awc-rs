use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::tile;

type MoveCost = u32;

const DEFAULT_COST : u32 = 1;

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
}