use serde::Deserialize;
use serde::Serialize;

use crate::event;
use crate::ID;
use crate::script::Script;


#[derive(Clone, Serialize, Deserialize)]
pub struct Effect
{
    pub type_id : ID,
    pub listens_to : Vec<(event::Notification, event::SubType)>,
    pub name : String,
    pub script : String,
}

impl Effect{
    pub fn notify(&self, script : &Script, notification : (event::Notification, event::SubType), event : &event::Event){
        if self.listens_to.contains(&notification){
            script.exec();
        }
    }
}

// Wake
// ID: 0
// Listens to: END_TURN
// Name: Wake
// Script: script (ID OR NAME)