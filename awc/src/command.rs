use crate::ID;
use crate::event;
use crate::event::Event;
use crate::game;
use crate::map;
use crate::movement;

pub trait CommandI{

    fn execute(&self, game : &mut game::Game) -> Result<(), Error>;
}

pub enum Command{
    Move(Move),
    Attack(Attack),
    Wait(Wait)
}

impl CommandI for Command{
    fn execute(&self, game : &mut game::Game) -> Result<(), Error> {
        match &self{
            Command::Move(m) => m.execute(game),
            Command::Attack(_) => todo!(),
            Command::Wait(w) => w.execute(game),
        }
    }
}

#[derive(Debug)]
pub enum Error{
    EntityNotFound(ID),
    MoveError(movement::Error)
}

impl From<movement::Error> for Error{
    fn from(e: movement::Error) -> Self {
        Error::MoveError(e)
    }
}

#[derive(new)]
pub struct Move{
    pub entity_id : ID,
    pub dest : map::Pos,
}

impl CommandI for Move{
    fn execute(&self, game : &mut game::Game) -> Result<(), Error>{
        
        let pos = game.components().get_position(&self.entity_id);
        if pos.is_some() {

            let (path, _cost) = movement::calc_path(game, self.entity_id, self.dest)?;
            if path.len() >= 2{
                for i in 1..path.len() {
                    let origin = path.iter().nth(i - 1).expect("Path didn't contain a single pos");
                    let dest = path.iter().nth(i).expect("Path didn't have a second pos");
                    let move_event = event::Move::new(self.entity_id, *origin, *dest);
                    game.push_event(Event::Move(move_event));
                }

                game.run_events();

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
    fn execute(&self, game : &mut game::Game) -> Result<(), Error> {
        
        let pos = game.components().get_position(&self.entity_id);
        if pos.is_some() {
            let wait_event = event::Wait::new(self.entity_id);
            game.push_event(Event::Wait(wait_event));
            game.run_events();

            return Ok(());
        }

        Err(Error::EntityNotFound(self.entity_id))
    }
}