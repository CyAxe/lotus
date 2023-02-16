use mlua::UserData;
use tealr::TypeName;

#[derive(TypeName, Debug)]
pub struct ResponseMatcher {}

impl ResponseMatcher {
    pub fn match_and_body(&self, body: String, text: Vec<String>) -> bool {
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
    pub fn match_once_body(&self, body: String, text: Vec<String>) -> String {
        let mut matched_data = "".into();
        text.iter().for_each(|x| {
            if body.contains(x) {
                matched_data = x.to_string();
            }
        });
        matched_data
    }
}

impl UserData for ResponseMatcher {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(
            "match_body",
            |_, this, (response, text_list): (String, Vec<String>)| {
                Ok(this.match_and_body(response, text_list))
            },
        );
        methods.add_method(
            "match_body_once",
            |_, this, (response, text_list): (String, Vec<String>)| {
                let is_match = this.match_once_body(response, text_list);
                Ok(is_match)
            },
        )
    }
}
