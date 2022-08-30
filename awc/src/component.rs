
use crate::{weapon, movement, unit, tile, player, map};

pub type EntityID = i32;

pub enum Component
{
    Type(Type),
    Position(Position),
    Direction(Direction),
    Health(Health),
    Weapons(Weapons),
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

pub struct Owner
{
    pub owner : player::ID,
}

pub struct CaptureState
{
    pub progress : i32,
}