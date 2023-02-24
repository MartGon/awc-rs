use serde::{Serialize, Deserialize};

use crate::{weapon, movement, effect, component::{self}};

pub type TypeID = super::ID;

#[derive(Serialize, Deserialize, Clone)]
pub struct Template
{
    pub weapons : Vec<weapon::Weapon>,
    pub movement : Option<movement::Movement>,
    pub effects : Vec<effect::Effect>,
}

impl Template{
    pub fn new(weapons : &[weapon::Weapon], movement : Option<movement::Movement>, effects : &[effect::Effect]) -> Template{
        Template { weapons : weapons.to_vec(), movement, effects : effects.to_vec() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Unit
{
    pub utype : component::Type,
    pub position : component::Position,
    pub health : component::Health,
    pub owner : component::Owner,
    pub direction : Option<component::Direction>,
    pub armament : Option<component::Armament>,
    pub movement : Option<component::Movement>,
    pub effects : Option<component::Effects>,
}

// Could replace this with an Enum. 
// EXTRA: Could use a mask as well, instead of a bunch of booleans
// Ready state means the unit can do anything
// Attacked state means the unit can still move
// Moved state means the unit can still attack
// Waiting state means the unit cannot do anything

pub struct State{
    pub has_attacked : bool,
    pub has_moved : bool,
    pub is_waiting : bool,
}