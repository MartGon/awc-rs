use crate::table::TableID;
use std::hash::Hash;

pub struct ID(i32);

impl TableID for ID{
    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl Default for ID{
    fn default() -> Self {
        Self(0)
    }
}

impl PartialEq for ID{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ID{
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for ID{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub type TeamID = ID;

pub struct Player
{
    pub id : ID,
    pub funds : i32,
    pub team : TeamID,
}

impl Player{
    pub fn new(id : ID, teamId : TeamID) -> Player{
        Player { id: id, funds: 0, team:  teamId}
    }
}