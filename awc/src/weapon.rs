use std::collections::HashMap;
use crate::unit;

pub struct Range
{
    min : i32,
    max : i32,
}

pub struct Weapon
{
    range : Range,
    dmg : i32,
    ammo : i32,
    max_ammo : i32,
    attackable_units : HashMap<unit::TypeID, bool>
}