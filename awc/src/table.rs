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

impl<ID : TableID + Default + Eq + Hash, T> Table<ID, T> {
    pub fn new() -> Table<ID, T>{
        Table { map: HashMap::new(), last_id : ID::default() }
    }

    pub fn new_entry(&mut self, t : T) -> ID{
        let id = self.last_id.next();
        self.map.insert(id, t);
        id
    }
    
}