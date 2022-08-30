use crate::{component, map, game};

pub enum Event
{
    UnitMove(UnitMove),
    UnitAttack(UnitAttack),
    UnitTakeDmg(UnitTakeDmg),
    UnitSpawn(UnitSpawn),
    UnitDie(UnitDie),
    StartTurn(StartTurn),
    EndTurn(EndTurn),
}

pub struct UnitMove
{
    pub unit : component::EntityID,
    pub origin : map::Pos,
    pub dest : map::Pos,
}

pub struct UnitAttack
{
    pub unit : component::EntityID,
    pub target : component::EntityID,
}

pub struct UnitTakeDmg
{
    pub attacker : component::EntityID,
    pub vicitim : component::EntityID,
    pub dmg_taken : i32,
}

pub struct UnitSpawn
{
    pub unit : component::EntityID,
    pub pos : map::Pos,
    pub spawned_by : Option<component::EntityID>
}

pub struct UnitDie
{
    pub unit : component::EntityID,
    pub killer : Option<component::EntityID>,
}

pub struct StartTurn
{
    pub turn : game::Turn,
}

pub struct EndTurn
{
    pub turn : game::Turn,
}