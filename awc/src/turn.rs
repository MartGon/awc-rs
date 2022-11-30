use std::collections::HashMap;

use crate::{player, ID};

pub struct Turn{
    day : u32,
    player : player::ID,
    waiting_entities : HashMap<ID, ()>,
}

impl Turn{

    pub fn new(day : u32, player : player::ID) -> Turn{
        Turn{day, player, waiting_entities : HashMap::new()}
    }

    pub fn entity_wait(&mut self, entity_id : ID) {
        self.waiting_entities.insert(entity_id, ());
    }

    pub fn is_waiting(&self, entity_id : ID) -> bool{
        self.waiting_entities.contains_key(&entity_id)
    }
}