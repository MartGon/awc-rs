use std::collections::HashMap;

use crate::{weapon, movement};

// TODO: Maybe could use generics for this module and tile. Merge on entity.rs


pub type TypeID = i32;

struct Template
{
    pub weapons : Vec<weapon::Weapon>,
    pub movement : Option<movement::Movement>,
}

// Provide interface a such as AddTemplate or CreateInstance by typeId.
pub struct Factory
{
    templates : HashMap<TypeID, Template>
}