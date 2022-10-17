use std::collections::HashMap;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

pub trait TableID{

    fn next(&self) -> Self;
}

#[derive(Deserialize, Serialize)]
pub struct Table<ID: Hash + Eq, T>
{
    map : HashMap<ID, T>,
    last_id : ID,
}

impl<ID : TableID + Default + Eq + Hash + Copy, T> Table<ID, T> {
    pub fn new() -> Table<ID, T>{
        Table { map: HashMap::new(), last_id : ID::default() }
    }

    pub fn new_entry(&mut self, t : T) -> ID{
        let id = self.next_id();
        self.map.insert(id, t);
        id
    }

    pub fn get_entry(&mut self, id : &ID) -> Option<&mut T>{
        self.map.get_mut(id)
    }
    
    pub fn next_id(&mut self) -> ID{
        self.last_id.next()
    }
}