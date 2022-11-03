use std::collections::HashMap;

pub type TypeID = super::ID;

pub struct Template
{
    pub capturable : bool,
}

pub struct Factory
{
    templates : HashMap<TypeID, Template>
}