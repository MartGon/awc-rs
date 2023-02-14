use crate::{component, map, game, turn::Turn, ID};

pub trait EventI {
    
    fn run(&self, game : &mut game::Game);
}

#[derive(Clone)]
pub enum Trigger{
    Event,
    PlayerCommand
}

// TODO: When do we assign an id?
//  Option 1: On Push to history
//      Problem: Difficulty to assign trigger id on PRE execution triggered event
//  Option 2: On creation. Game provides interface to create events or give a free id
//      // Problem 1: Cancelled events consume ids, but are never used again on a post increment method
//      // Problem 2: Ids may not match execution order
//      // Solultion: Contain events in a table. Use a vector to hold history of event ids in execution order. Oldest first

#[derive(Clone)]
pub struct Event{
    pub id : ID,
    pub event_type : EventType,
    pub trigger : Trigger,
}

#[derive(Clone)]
pub enum EventType
{
    Move(Move),
    Attack(Attack),
    Wait(Wait),
    TakeDmg(TakeDmg),
    Spawn(Spawn),
    Die(Die),
    StartTurn(StartTurn),
    EndTurn(EndTurn),
}

impl Event{
    pub(crate) fn new(event_type : EventType, trigger : Trigger) -> Event{
        Event { id: ID::default(), event_type, trigger }
    }
}

impl EventI for Event{
    fn run(&self, game : &mut game::Game) {
        match &self.event_type{
            EventType::Move(m) => m.run(game),
            EventType::Attack(_) => todo!(),
            EventType::TakeDmg(_) => todo!(),
            EventType::Spawn(_) => todo!(),
            EventType::Die(_) => todo!(),
            EventType::StartTurn(_) => todo!(),
            EventType::EndTurn(e) => e.run(game),
            EventType::Wait(w) => w.run(game),
        }
    }
}

#[derive(new, Clone)]
pub struct Move
{
    pub unit : component::EntityID,
    pub origin : map::Pos,
    pub dest : map::Pos,
}

impl EventI for Move{
    fn run(&self, game : &mut game::Game) {
        let mut pos = game.components_mut().get_position_mut(&self.unit).expect("Entity didn't have a pos");
        pos.pos = self.dest;
    }
}

#[derive(Clone)]
pub struct Attack
{
    pub unit : component::EntityID,
    pub target : component::EntityID,
}

#[derive(new, Clone)]
pub struct Wait
{
    pub entity_id : component::EntityID,
}

impl EventI for Wait{
    fn run(&self, game : &mut game::Game) {
        let turn = game.get_turn_mut().expect("Game hasn't started");
        turn.entity_wait(self.entity_id);
    }
}

#[derive(Clone)]
pub struct TakeDmg
{
    pub attacker : component::EntityID,
    pub victim : component::EntityID,
    pub dmg_taken : i32,
}

#[derive(Clone)]
pub struct Spawn
{
    pub unit : component::EntityID,
    pub pos : map::Pos,
    pub spawned_by : Option<component::EntityID>
}

#[derive(Clone)]
pub struct Die
{
    pub unit : component::EntityID,
    pub killer : Option<component::EntityID>,
}

#[derive(Clone)]
pub struct StartTurn
{
    pub turn : Turn,
}

#[derive(Clone)]
pub struct EndTurn
{
    pub turn : Turn,
}

impl EventI for EndTurn{
    fn run(&self, game : &mut game::Game) {
        game.end_turn();
    }
}