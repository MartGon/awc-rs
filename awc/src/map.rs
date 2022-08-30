use crate::component;

pub struct Size
{
    pub width : i32,
    pub height : i32
}

pub struct Pos
{
    pub x : i32,
    pub y : i32,
    pub z : i32,
}

pub struct Map
{
    tiles : Vec<component::EntityID>,
    pub size : Size,
}