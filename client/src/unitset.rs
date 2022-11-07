use std::collections::HashMap;

use awc::{*, player::Team};

use crate::spritesheet::AnimatedSprite;

pub type UnitSet = HashMap<unit::TypeID, Unit>; 


#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Faction{
    OrangeStar = 0,
    BlueMoon,
    YellowComet,
    GreenEarth,
    BlackHole
}

pub struct Unit{
    sprites : HashMap<(Team, Faction), AnimatedSprite>,
}

impl Unit{

    pub fn new(sprites: &[((Team, Faction), AnimatedSprite)]) -> Unit{
        Unit { sprites: sprites.into_iter().cloned().collect() }
    }

    pub fn sprite(&self, team_id : Team) -> Option<&AnimatedSprite>{
        self.sprite_faction(team_id, Faction::OrangeStar)
    }

    pub fn sprite_faction(&self, team_id : Team, faction : Faction) -> Option<&AnimatedSprite>{
        self.sprites.get(&(team_id, faction))
    }
}