use crate::lua::network::http::HttpResponse;
use mlua::UserData;
use tealr::TypeName;

#[derive(TypeName, Debug)]
pub struct ResponseMatcher {}

impl ResponseMatcher {
    pub fn match_and_body(&self, response: HttpResponse, text: Vec<String>) -> bool {
        let body = response.body;
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
        methods.add_method(
            "match_body",
            |_, this, (response, text_list): (HttpResponse, Vec<String>)| {
                Ok(this.match_and_body(response, text_list))
            },
        );
    }
}
