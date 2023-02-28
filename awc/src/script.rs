use std::{fs, fmt};

use mlua::prelude::*;

#[derive(Debug)]
pub enum Error{
    FileNotFound(std::io::Error, String),
    CompileError(mlua::Error, String)
}   

impl fmt::Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::FileNotFound(e, s) => write!(f, "{}: {}", e, s),
            Self::CompileError(e, s) => write!(f, "{}: {}", e, s),
        }
    }
}

pub struct Script<'a>{
    pub name : String,
    func : LuaFunction<'a>
}

impl<'a : 'b, 'b> Script<'a>{

    pub fn from_file<P: AsRef<std::path::Path> + Into<String> + Clone>(lua : &'a Lua, name : String, file : P) -> Result<Script<'b>, self::Error>{
        let code = fs::read_to_string(&file).map_err(|e| self::Error::FileNotFound(e, file.clone().into()))?;
        let func = lua.load(&code).into_function().map_err(|e| self::Error::CompileError(e, file.clone().into()))?;
        let script = Script{
            name,
            func
        };
        Ok(script)
    }

    pub fn exec(&self, ){
        self.func.call::<_, ()>(()).expect(format!("Script code is incorrect. Script name: {}", self.name).as_str());
    }

}