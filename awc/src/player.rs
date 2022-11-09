use serde::{Serialize, Deserialize};

pub type ID = super::ID;

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Team{
    Red = 0,
    Blue,
    Green,
    Yellow
}

pub struct Player
{
    pub id : ID,
    pub funds : i32,
    pub team : Team,
}

impl Player{
    pub fn new(id : ID, team : Team) -> Player{
        Player { id: id, funds: 0, team:  team}
    }
}