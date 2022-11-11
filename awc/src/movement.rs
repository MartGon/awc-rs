use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::tile;

type MoveCost = i32;

#[derive(Serialize, Deserialize, Clone)]
pub struct Movement
{
    range : i32,
    traverse_cost : HashMap<tile::TypeID, MoveCost>
}