use std::collections::HashMap;

use crate::{weapon, movement, component};

// TODO: Maybe could use generics for this module and tile. Merge on entity.rs


pub type TypeID = super::ID;

pub struct Template
{
    pub weapons : Vec<weapon::Weapon>,
    pub movement : Option<movement::Movement>,
}

struct Unit
{
    pub utype : component::Type,
    pub position : component::Position,
    pub health : component::Health,
    pub direction : Option<component::Direction>,
    pub armament : Option<component::Armament>,
    pub movement : Option<component::Movement>,
    pub owner : Option<component::Owner>,
}

// Provide interface a such as AddTemplate or CreateInstance by typeId.
pub struct Factory
{
    templates : HashMap<TypeID, Template>
}