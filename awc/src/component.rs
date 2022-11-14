
use crate::{weapon, movement, unit, tile, player, map::{self, Pos}, ID};
use component_derive::ComponentCollection;
use serde::{Serialize, Deserialize};

pub type EntityID = super::ID;

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

#[derive(Serialize, Deserialize, Clone)]
pub enum EntityType
{
    Unit = 0,
    Tile
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Type
{
    pub type_id : ID,
    pub entity_type : EntityType
}

impl Type{

    pub fn new(type_id : ID, entity_type : EntityType) -> Type{
        Type{type_id, entity_type}
    }

    pub fn new_unit(type_id : ID)-> Type{
        Type{type_id, entity_type : EntityType::Unit}
    }

    pub fn new_tile(type_id : ID)-> Type{
        Type{type_id, entity_type : EntityType::Tile}
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Position
{
    pub pos : map::Pos
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Dir
{
    North,
    South,
    East,
    West
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Direction
{
    pub direction : Dir
}

impl Default for Direction{
    fn default() -> Direction{
        Direction{ direction : Dir::North }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Health
{
    pub health : f32
}

impl Default for Health{
    fn default() -> Self {
        Self { health: 100.0 }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Armament
{
    pub weapons : Vec<weapon::Weapon>,
}

impl Armament{
    pub fn new(weapons : Vec<weapon::Weapon>) -> Armament{
        Armament { weapons }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Movement
{
    pub movement : movement::Movement,
    pub gas : i32,
    pub max_gas : i32,
}

impl Movement{
    pub fn new(movement : movement::Movement) -> Movement{
        Movement { movement, gas: 100, max_gas: 100 }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Owner
{
    pub owner : player::ID,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CaptureState
{
    pub progress : i32,
}