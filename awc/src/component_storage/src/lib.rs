use std::{hash::Hash, collections::{HashMap, hash_map::IntoIter}};

pub struct ComponentStorage<ID: Hash + Eq, T>
{
    components: HashMap<ID, T>
}

impl<ID: Hash + Eq, T> Default for ComponentStorage<ID, T>{
    fn default() -> Self {
        Self::new()
    }
}

impl<Id: Hash + Eq, V> ComponentStorage<Id, V> {

    pub fn new() -> Self{
        Self{components : HashMap::new()}
    }

    pub fn insert(&mut self, id : Id, component : V) -> Option<V>{
        self.components.insert(id, component)
    }

    pub fn entry_mut(&mut self, id : &Id) -> Option<&mut V>{
        self.components.get_mut(&id)
    }

    pub fn entry(&self, id : &Id) -> Option<&V>{
        self.components.get(&id)
    }
}

impl<'a, ID: Hash + Eq, V>IntoIterator for &'a ComponentStorage<ID, V>{
    type Item = (&'a ID, &'a V);
    type IntoIter = std::collections::hash_map::Iter<'a, ID, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.iter()
    }
}

impl<ID: Hash + Eq, V>IntoIterator for ComponentStorage<ID, V>{
    type Item = (ID, V);
    type IntoIter = std::collections::hash_map::IntoIter<ID, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
    }
}