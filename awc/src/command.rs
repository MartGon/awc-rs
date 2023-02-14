use crate::ID;
use crate::event;
use crate::event::Event;
use crate::event::SubEvent;
use crate::event::Trigger;
use crate::game;
use crate::map;
use crate::movement;
use crate::player;

pub trait CommandI{

    fn execute(&self, game : &mut game::Game, author : &player::ID) -> Result<(), Error>;
}

pub enum Command{
    Move(Move),
    Attack(Attack),
    Wait(Wait),
    EndTurn(EndTurn)
}

impl CommandI for Command{
    fn execute(&self, game : &mut game::Game, author : &player::ID) -> Result<(), Error> {
        match &self{
            Command::Move(m) => m.execute(game, author),
            Command::Attack(_) => todo!(),
            Command::Wait(w) => w.execute(game, author),
            Command::EndTurn(e) => e.execute(game, author)
        }
    }
}

#[derive(Debug)]
pub enum Error{
    EntityNotFound(ID),
    EntityIsWaiting(ID),
    EntityNotOwnedByPlayer(ID, ID),
    MoveError(movement::Error)
}

impl From<movement::Error> for Error{
    fn from(e: movement::Error) -> Self {
        Error::MoveError(e)
    }
}

fn check_owner(game : &mut game::Game, author : &player::ID, entity_id : &ID) -> Result<(), Error>{
    if let Some(owner) = game.components().get_owner(entity_id){
        if owner.owner != *author{
            return Err(Error::EntityNotOwnedByPlayer(*entity_id, *author));
        }
        else {
            return Ok(())
        }
    }

    return Err(Error::EntityNotOwnedByPlayer(*entity_id, *author));
}

#[derive(new)]
pub struct Move{
    pub entity_id : ID,
    pub dest : map::Pos,
}

impl CommandI for Move{
    fn execute(&self, game : &mut game::Game, author : &player::ID) -> Result<(), Error>{
        
        check_owner(game, author, &self.entity_id)?;

        let pos = game.components().get_position(&self.entity_id);
        if pos.is_some() {

            let turn = game.current_turn();

            if turn.is_waiting(self.entity_id){
                return Err(Error::EntityIsWaiting(self.entity_id));
            }

            let (path, _cost) = movement::calc_path(game, self.entity_id, self.dest)?;
            if path.len() >= 2{
                for i in 1..path.len() {
                    let origin = path.iter().nth(i - 1).expect("Path didn't contain a single pos");
                    let dest = path.iter().nth(i).expect("Path didn't have a second pos");
                    let move_event = event::Move::new(self.entity_id, *origin, *dest);
                    let event = Event::new(SubEvent::Move(move_event), Trigger::PlayerCommand);
                    game.push_event(event);
                }

                return Ok(());
            }
            else {
                return Err(movement::Error::DestinationSameAsOrigin.into());
            }
        }

        Err(Error::EntityNotFound(self.entity_id))
    }
}

pub struct Attack{

}

#[derive(new)]
pub struct Wait{
    pub entity_id : ID,
}

impl CommandI for Wait{
    fn execute(&self, game : &mut game::Game, author : &player::ID) -> Result<(), Error> {
        
        check_owner(game, author, &self.entity_id)?;

        let pos = game.components().get_position(&self.entity_id);
        if pos.is_some() {
            let wait_event = event::Wait::new(self.entity_id);
            let event = Event::new(SubEvent::Wait(wait_event), Trigger::PlayerCommand);
            game.push_event(event);

            return Ok(());
        }

        Err(Error::EntityNotFound(self.entity_id))
    }
}

#[derive(new)]
pub struct EndTurn{

}

impl CommandI for EndTurn{
    fn execute(&self, game : &mut game::Game, _author : &player::ID) -> Result<(), Error> {

        let end_turn = event::EndTurn{turn : game.current_turn().clone()};
        let event = Event::new(SubEvent::EndTurn(end_turn), Trigger::PlayerCommand);
        game.push_event(event);

        return Ok(());
    }
}