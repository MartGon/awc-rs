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
    next_id : ID,
}

impl<ID : TableID + Default + Eq + Hash + Copy + AsRef<u32>, T> Table<ID, T> {
    pub fn new() -> Table<ID, T>{
        Table { map: HashMap::new(), next_id : ID::default() }
    }

    pub fn new_entry(&mut self, t : T) -> ID{
        let id = self.next_id;
        self.map.insert(id, t);
        self.advance_id();
        id
    }

    pub fn get_entry(&self, id : &ID) -> Option<&T>{
        self.map.get(id)
    }

    pub fn get_entry_mut(&mut self, id : &ID) -> Option<&mut T>{
        self.map.get_mut(id)
    }
    
    pub fn next_id(&mut self) -> ID{
        self.next_id
    }

    fn advance_id(&mut self){
        self.next_id = self.next_id.next();
    }
}