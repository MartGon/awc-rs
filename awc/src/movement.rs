use std::collections::HashMap;

use crate::tile;

type MoveCost = i32;

pub struct Movement
{
    range : i32,
    traverse_cost : HashMap<tile::TypeID, MoveCost>
}