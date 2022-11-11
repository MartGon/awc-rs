use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::unit;

#[derive(Serialize, Deserialize, Clone)]
pub struct Range
{
    min : i32,
    max : i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Weapon
{
    range : Range,
    dmg : i32,
    ammo : i32,
    max_ammo : i32,
    attackable_units : HashMap<unit::TypeID, bool>
}