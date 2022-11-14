use std::collections::HashMap;
use crate::ID;

pub trait Instance{

}

pub trait Template<I: Instance> {
    
    fn create_instance(&self, id : &ID) -> I;
}

pub trait FactoryI<T: Template<I>, I:Instance> {
    
    fn create_instance(&self, id : &ID) -> Option<I>;
}

pub struct Factory<T>
{
    templates : HashMap<ID, T>
}

impl<T> Factory<T>{

    pub fn new() -> Factory<T>{
        Factory { templates:  HashMap::new() }
    }

    pub fn add_template(&mut self, id : &ID, template : T){
        self.templates.insert(*id, template);
    }
}

impl<T: Template<I>, I: Instance> FactoryI <T, I> for Factory<T>{
    fn create_instance(&self, id : &ID) -> Option<I> {
        if let Some(template) = self.templates.get(id){
            let instance = template.create_instance(id);
            Some(instance)
        }
        else{
            None
        }
    }
}