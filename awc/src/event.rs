use crate::{component, map, game, turn::Turn};

pub trait EventI {
    
    fn run(&self, game : &mut game::Game);
}

pub enum Event
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

impl EventI for Event{
    fn run(&self, game : &mut game::Game) {
        match &self{
            Event::Move(m) => m.run(game),
            Event::Attack(_) => todo!(),
            Event::TakeDmg(_) => todo!(),
            Event::Spawn(_) => todo!(),
            Event::Die(_) => todo!(),
            Event::StartTurn(_) => todo!(),
            Event::EndTurn(_) => todo!(),
            Event::Wait(w) => w.run(game),
        }
    }
}

#[derive(new)]
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

pub struct Attack
{
    pub unit : component::EntityID,
    pub target : component::EntityID,
}

#[derive(new)]
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

pub struct TakeDmg
{
    pub attacker : component::EntityID,
    pub vicitim : component::EntityID,
    pub dmg_taken : i32,
}

pub struct Spawn
{
    pub unit : component::EntityID,
    pub pos : map::Pos,
    pub spawned_by : Option<component::EntityID>
}

pub struct Die
{
    pub unit : component::EntityID,
    pub killer : Option<component::EntityID>,
}

pub struct StartTurn
{
    pub turn : Turn,
}

pub struct EndTurn
{
    pub turn : Turn,
}