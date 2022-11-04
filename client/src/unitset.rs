use std::collections::HashMap;

use awc::{*, player::TeamID};

use crate::spritesheet::AnimatedSprite;

pub type UnitSet = HashMap<unit::TypeID, Unit>; 


#[derive(PartialEq, Eq, Hash)]
pub enum SpriteVariant{
    OrangeStar = 0,
    BlueMoon,
    YellowComet,
    GreenEarth,
    BlackHole
}

pub struct Unit{
    sprites : HashMap<(awc::player::TeamID, SpriteVariant), AnimatedSprite>,
}

impl Unit{

    pub fn sprite(&self, team_id : TeamID) -> Option<&AnimatedSprite>{
        self.sprites.get(&(team_id, SpriteVariant::OrangeStar))
    }
}