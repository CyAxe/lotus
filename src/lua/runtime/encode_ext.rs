use crate::lua::encode::base64::{base64_decode, base64_encode};
use crate::lua::encode::html::{htmldecode, htmlencode};
use crate::lua::encode::url::{urldecode, urlencode};
use crate::lua::model::LuaRunTime;

pub trait EncodeEXT {
    fn add_encode_function(&self);
}

impl EncodeEXT for LuaRunTime<'_> {
    fn add_encode_function(&self) {
        self.lua
            .globals()
            .set(
                "base64encode",
                self.lua.create_function(base64_encode).unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "base64decode",
                self.lua.create_function(base64_decode).unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set("urlencode", self.lua.create_function(urlencode).unwrap())
            .unwrap();
        self.lua
            .globals()
            .set("urldecode", self.lua.create_function(urldecode).unwrap())
            .unwrap();
        self.lua
            .globals()
            .set("htmlencode", self.lua.create_function(htmlencode).unwrap())
            .unwrap();
        self.lua
            .globals()
            .set("htmldecode", self.lua.create_function(htmldecode).unwrap())
            .unwrap();
    }
}
