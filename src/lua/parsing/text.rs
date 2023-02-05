use crate::lua::network::http::HttpResponse;
use mlua::UserData;
use tealr::TypeName;

#[derive(TypeName, Debug)]
pub struct ResponseMatcher {
    pub response: HttpResponse,
}

impl ResponseMatcher {
    pub fn init(response: HttpResponse) -> ResponseMatcher {
        ResponseMatcher { response }
    }
    pub fn change_response(&mut self, response: HttpResponse) {
        self.response = response;
    }
    pub fn match_and_body(&self, text: Vec<String>) -> bool {
        let body = &self.response.body;
        let mut counter = 0;
        text.iter().for_each(|x| {
            if body.contains(x) {
                counter += 1;
            }
        });
        if counter == text.len() {
            true
        } else {
            false
        }
    }
}

impl UserData for ResponseMatcher {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("match_body", |_, this, text_list: Vec<String>| {
            Ok(this.match_and_body(text_list))
        });
        methods.add_method_mut("change_response", |_, this, response: HttpResponse| {
            this.change_response(response);
            Ok(())
        });
    }
}
