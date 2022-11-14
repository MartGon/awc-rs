use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::unit;

#[derive(Serialize, Deserialize, Clone)]
pub struct Range
{
    min : u32,
    max : u32,
}

impl Range{
    pub fn new(min : u32, max : u32) -> Range{
        Range { min, max }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Weapon
{
    range : Range,
    dmg : u32,
    ammo : u32,
    max_ammo : u32,
    attackable_units : HashMap<unit::TypeID, bool>
}

impl Weapon{
    pub fn new(range : Range, dmg : u32, max_ammo : u32, attackable_units : &[(unit::TypeID, bool)]) -> Weapon{
        Weapon { range, dmg, ammo : max_ammo, max_ammo, attackable_units : attackable_units.into_iter().cloned().collect() }
    }

    pub fn can_attack_unit(&self, victim : unit::TypeID) -> bool{
        if let Some(attackable) = self.attackable_units.get(&victim){
            *attackable
        }
        else{
            false
        }
    }
}