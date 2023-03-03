use crate::LuaRunTime;

pub trait EncodeEXT {
    fn add_encode_function(&self);
}


impl EncodeEXT for LuaRunTime<'_> {
    fn add_encode_function(&self) {
        self.lua
            .globals()
            .set(
                "base64encode",
                self.lua
                    .create_function(|_, data: String| Ok(base64::encode(data)))
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "base64decode",
                self.lua
                    .create_function(|_, data: String| Ok(base64::decode(data).unwrap()))
                    .unwrap(),
            )
            .unwrap();
    }
}

