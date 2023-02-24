use serde::{Serialize, Deserialize};

use crate::{component, map, game, turn::Turn, ID};

pub trait EventI {
    
    fn run(&self, game : &mut game::Game);
}

#[derive(Clone)]
pub enum Trigger{
    Event,
    PlayerCommand
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Notification{
    Pre,
    Post
}

#[derive(Clone)]
pub struct Event{
    pub id : ID,
    pub sub_event : SubEvent,
    pub trigger : Trigger,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubType
{
    Move = 0,
    Attack,
    Wait,
    Wake,
    TakeDmg,
    Spawn,
    Die,
    StartTurn,
    EndTurn,
}


#[derive(Clone)]
pub enum SubEvent
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

impl SubEvent{
    pub fn sub_type(&self) -> SubType {
        match self {
            SubEvent::Move(_) => SubType::Move,
            SubEvent::Attack(_) => SubType::Attack,
            SubEvent::Wait(_) => SubType::Wait,
            SubEvent::TakeDmg(_) => SubType::TakeDmg,
            SubEvent::Spawn(_) => SubType::Spawn,
            SubEvent::Die(_) => SubType::Die,
            SubEvent::StartTurn(_) => SubType::StartTurn,
            SubEvent::EndTurn(_) => SubType::EndTurn,
        }
    }
}


impl Event{
    pub(crate) fn new(sub_event : SubEvent, trigger : Trigger) -> Event{
        Event { id: ID::default(), sub_event, trigger }
    }
}

impl EventI for Event{
    fn run(&self, game : &mut game::Game) {
        match &self.sub_event{
            SubEvent::Move(m) => m.run(game),
            SubEvent::Attack(_) => todo!(),
            SubEvent::TakeDmg(_) => todo!(),
            SubEvent::Spawn(_) => todo!(),
            SubEvent::Die(_) => todo!(),
            SubEvent::StartTurn(_) => todo!(),
            SubEvent::EndTurn(e) => e.run(game),
            SubEvent::Wait(w) => w.run(game),
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