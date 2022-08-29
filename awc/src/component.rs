
use crate::{weapon, movement};

pub enum Component
{
    Position(Position),
    Direction(Direction),
    Health(Health),
    Weapons(Weapons),
    Movement(Movement)
}

pub struct Position
{
    pub x : i32,
    pub y : i32,
    pub z : i32
}

pub enum Dir
{
    North,
    South,
    East,
    West
}

pub struct Direction
{
    pub direction : Dir
}

pub struct Health
{
    pub health : f32
}

pub struct Weapons
{
    pub weapons : Vec<weapon::Weapon>,
}

pub struct Movement
{
    pub movement : movement::Movement,
    pub gas : i32,
    pub max_gas : i32,
}