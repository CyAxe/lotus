use crate::lua::parsing::url::HttpMessage;
use mlua::{ExternalError, ExternalResult, UserData};
use url::Url;

impl UserData for HttpMessage {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("clone", |_,this,()|{
            let msg = this.clone();
            Ok(msg)
        });
        methods.add_method_mut("new", |_, this, url: String| match Url::parse(&url) {
            Ok(parsed_url) => {
                this.url = Some(parsed_url.clone());
                Ok(parsed_url.to_string())
            }
            Err(err) => Err(err.to_lua_err()),
        });
        methods.add_method(
            "param_set",
            |_, this, (param, payload, remove_content): (String, String, bool)| {
                Ok(this.set_urlvalue(&param, &payload, remove_content))
            },
        );
        methods.add_method(
            "param_set_all",
            |_, this, (payload, remove_content): (String, bool)| {
                Ok(this.change_urlquery(&payload, remove_content))
            },
        );
        methods.add_method("url", |_, this, ()| match &this.url {
            Some(url) => Ok(url.as_str().to_string()),
            None => Err("No url found").to_lua_err(),
        });
        methods.add_method("path", |_, this, ()| match &this.url {
            Some(url) => Ok(url.path().to_string()),
            None => Err("No url found").to_lua_err(),
        });
        methods.add_method("param_str", |_, this, ()| match &this.url {
            Some(url) => Ok(url.query().unwrap_or("").to_string()),
            None => Err("No url found").to_lua_err(),
        });
        methods.add_method("param_list", |_, this, ()| {
            let mut all_params = Vec::new();
            match &this.url {
                Some(url) => {
                    url.query_pairs().for_each(|(param_name, _param_value)| {
                        all_params.push(param_name.to_string());
                    });
                    Ok(all_params)
                }
                None => Err("No url found").to_lua_err(),
            }
        });
        methods.add_method("urljoin", |_, this, new_path: String| {
            Ok(this.urljoin(new_path.as_str()))
        });
    }
}
