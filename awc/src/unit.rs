use serde::{Serialize, Deserialize};

use crate::{weapon, movement, component::{self}, template::{Instance}};
use crate::template;

pub type TypeID = super::ID;

#[derive(Serialize, Deserialize)]
pub struct Template
{
    pub weapons : Vec<weapon::Weapon>,
    pub movement : Option<movement::Movement>,
}

impl Template{
    pub fn new(weapons : &[weapon::Weapon], movement : Option<movement::Movement>) -> Template{
        Template { weapons : weapons.into_iter().cloned().collect(), movement }
    }
}

impl template::Template<Unit> for Template{
    fn create_instance(&self, id : &crate::ID) -> Unit {
        Unit { 
            utype: component::Type::new_unit(*id), 
            position: component::Position::default(), 
            health: component::Health::default(), 
            owner: component::Owner::default(), 
            direction: None, 
            armament: if self.weapons.is_empty() { None } else { Some(component::Armament::new(self.weapons.clone()))}, 
            movement: if let Some(movement) = self.movement.clone() {Some(component::Movement::new(movement))} else { None }
        }
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

impl Instance for Unit{

}