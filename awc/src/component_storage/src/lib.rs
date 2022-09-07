use std::{hash::Hash, collections::HashMap};

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}