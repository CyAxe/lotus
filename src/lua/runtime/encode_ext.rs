use crate::lua::encode::base64::{base64_decode, base64_encode};
use crate::lua::encode::html::{htmldecode, htmlencode};
use crate::lua::encode::url::{urldecode, urlencode};
use crate::lua::model::LuaRunTime;
use mlua::{Lua, Result, Table};

fn create_lotus_encode_table(lua: &Lua) -> Result<Table> {
    let globals = lua.globals();

    let lotus = match globals.get::<_, Table>("lotus") {
        Ok(table) => table,
        Err(_) => {
            let table = lua.create_table()?;
            globals.set("lotus", table.clone())?;
            table
        }
    };

    let encode_table = match lotus.get::<_, Table>("encode") {
        Ok(table) => table,
        Err(_) => {
            let table = lua.create_table()?;
            lotus.set("encode", table.clone())?;
            table
        }
    };

    Ok(encode_table)
}

pub trait EncodeEXT {
    fn add_encode_function(&self) -> Result<()>;
}

impl EncodeEXT for LuaRunTime<'_> {
    fn add_encode_function(&self) -> Result<()> {
        let encode_table = create_lotus_encode_table(&self.lua)?;

        let base64_encode_fn = self.lua.create_function(base64_encode)?;
        encode_table.set("base64encode", base64_encode_fn)?;

        let base64_decode_fn = self.lua.create_function(base64_decode)?;
        encode_table.set("base64decode", base64_decode_fn)?;

        let url_encode_fn = self.lua.create_function(urlencode)?;
        encode_table.set("urlencode", url_encode_fn)?;

        let url_decode_fn = self.lua.create_function(urldecode)?;
        encode_table.set("urldecode", url_decode_fn)?;

        let html_encode_fn = self.lua.create_function(htmlencode)?;
        encode_table.set("htmlencode", html_encode_fn)?;

        let html_decode_fn = self.lua.create_function(htmldecode)?;
        encode_table.set("htmldecode", html_decode_fn)?;

        Ok(())
    }
}
