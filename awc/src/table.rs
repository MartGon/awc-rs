use std::collections::HashMap;
use std::hash::Hash;


pub trait TableID{

    fn next(&self) -> Self;
}

pub struct Table<ID, T>
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
    
    pub fn next_id(&mut self) -> ID{
        self.last_id.next()
    }
}