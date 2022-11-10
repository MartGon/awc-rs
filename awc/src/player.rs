use serde::{Serialize, Deserialize};

pub type ID = super::ID;

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy, Debug)]
pub enum Team{
    Red = 0,
    Blue,
    Green,
    Yellow
}


#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy, Debug)]
pub enum Faction{
    OrangeStar = 0,
    BlueMoon,
    YellowComet,
    GreenEarth,
    BlackHole
}

pub struct Player
{
    pub id : ID,
    pub funds : i32,
    pub team : Team,
    pub faction : Faction,
}

impl Player{
    pub fn new(id : ID, team : Team, faction : Faction) -> Player{
        Player { id, funds: 0, team, faction}
    }
}