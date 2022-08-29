use crate::component;

pub struct Size
{
    pub width : i32,
    pub height : i32
}

pub struct Map
{
    tiles : Vec<component::EntityID>,
    pub size : Size,
}