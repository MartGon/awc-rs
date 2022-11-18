use serde::{Serialize, Deserialize};

use crate::{weapon, movement, component::{self}};

pub type TypeID = super::ID;

#[derive(Serialize, Deserialize, Clone)]
pub struct Template
{
    pub weapons : Vec<weapon::Weapon>,
    pub movement : Option<movement::Movement>,
}

impl Template{
    pub fn new(weapons : &[weapon::Weapon], movement : Option<movement::Movement>) -> Template{
        Template { weapons : weapons.to_vec(), movement }
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
}