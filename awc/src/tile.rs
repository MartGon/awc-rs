use std::collections::HashMap;

pub type TypeID = i32;

struct Template
{
    pub capturable : bool,
}

pub struct Factory
{
    templates : HashMap<TypeID, Template>
}