use serde::Deserialize;
use serde::Serialize;

use crate::event;
use crate::ID;
use crate::game::GameState;
use crate::script::Script;
use crate::game::Game;


#[derive(Clone, Serialize, Deserialize)]
pub struct Effect
{
    pub type_id : ID,
    pub listens_to : Vec<(event::Notification, event::SubType)>,
    pub name : String,
    pub script : String,
}

impl Effect{
    pub fn notify(&self, game : &mut GameState, script : &Script, not_type : event::Notification, event : &event::Event){
        let notification = (not_type, event.sub_event.sub_type());
        if self.listens_to.contains(&notification){
            script.exec(game);
        }
    }
}

// Wake
// ID: 0
// Listens to: END_TURN
// Name: Wake
// Script: script (ID OR NAME)