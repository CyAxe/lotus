use crate::LuaRunTime;
use crate::lua::payloads;
use crate::lua::parsing::html::Location;

pub trait PayloadsEXT {
    fn add_payloadsfuncs(&self);
}

impl PayloadsEXT for LuaRunTime<'_> {
    fn add_payloadsfuncs(&self) {
        self.lua
            .globals()
            .set(
                "XSSGenerator",
                self.lua
                    .create_function(
                        |_, (response, location, payload): (String, Location, String)| {
                            let xss_gen =
                                payloads::xss::PayloadGen::new(response, location, payload);
                            Ok(xss_gen.analyze())
                        },
                    )
                    .unwrap(),
            )
            .unwrap();
    }
}
