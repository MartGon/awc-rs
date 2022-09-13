pub type ID = super::ID;
pub type TeamID = super::ID;

pub struct Player
{
    pub id : ID,
    pub funds : i32,
    pub team : TeamID,
}

impl Player{
    pub fn new(id : ID, team : TeamID) -> Player{
        Player { id: id, funds: 0, team:  team}
    }
}