use crate::{component};

pub type TypeID = super::ID;

pub struct Template
{
    pub capturable : bool,
}

impl Template{
    fn _create_instance(&self, id : &crate::ID) -> Tile {
        Tile { 
            utype: component::Type::new_tile(*id), 
            position : component::Position::default(), 
            capture_state : if self.capturable {Some(component::CaptureState::default())} else {None} 
        }
    }
}

pub struct Tile
{
    pub utype : component::Type,
    pub position : component::Position,
    pub capture_state : Option<component::CaptureState>,
}