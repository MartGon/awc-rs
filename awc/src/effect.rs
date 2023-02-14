use crate::event;
use crate::ID;


pub struct Effect
{
    pub type_id : ID,
    pub listens_to : Vec<(event::Notification, event::SubType)>,
    pub name : String,
}

impl Effect{
    fn notify(&self, notification : (event::Notification, event::SubType)){
        if self.listens_to.contains(&notification){
            
        }
    }
}

// Wake
// ID: 0
// Listens to: END_TURN
// Name: Wake
// Script: script (ID OR NAME)