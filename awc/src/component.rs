
use crate::{weapon, movement, unit, tile, player, map};
use component_derive::ComponentCollection;

pub type EntityID = i32;

#[derive(ComponentCollection)]
pub enum Component
{
    Type(Type),
    Position(Position),
    Health(Health),
    Direction(Direction),
    Armament(Armament),
    Movement(Movement),
    Owner(Owner),
    CaptureState(CaptureState)
}

pub enum EntityType
{
    Unit(unit::TypeID),
    Tile(tile::TypeID)
}

pub struct Type
{
    pub entity_type : EntityType
}

pub struct Position
{
    pub pos : map::Pos
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

pub struct Armament
{
    pub weapons : Vec<weapon::Weapon>,
}

pub struct Movement
{
    pub movement : movement::Movement,
    pub gas : i32,
    pub max_gas : i32,
}

pub struct Owner
{
    pub owner : player::ID,
}

pub struct CaptureState
{
    pub progress : i32,
}