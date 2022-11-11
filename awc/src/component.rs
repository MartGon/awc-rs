
use crate::{weapon, movement, unit, tile, player, map};
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
    Unit(unit::TypeID),
    Tile(tile::TypeID)
}

impl EntityType{

    pub fn unit_type(&self) -> unit::TypeID{
        if let EntityType::Unit(type_id) = self{
            type_id.clone()
        }
        else{
            panic!("Entity was not a unit")
        }
    }

    pub fn tile_type(&self) -> unit::TypeID{
        if let EntityType::Tile(type_id) = self{
            type_id.clone()
        }
        else{
            panic!("Entity was not a tile")
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Type
{
    pub entity_type : EntityType
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Movement
{
    pub movement : movement::Movement,
    pub gas : i32,
    pub max_gas : i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Owner
{
    pub owner : player::ID,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CaptureState
{
    pub progress : i32,
}