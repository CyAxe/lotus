use crate::lua::parsing::url::HttpMessage;
use mlua::UserData;
use std::collections::HashMap;

impl UserData for HttpMessage {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("setParam", |_, this, (param, payload,remove_content): (String, String, bool)| {
            Ok(this.set_urlvalue(&param, &payload, remove_content))
        });
        methods.add_method(
            "setAllParams",
            |_, this, (payload, remove_content): (String, bool)| {
                Ok(this.change_urlquery(&payload, remove_content))
            },
        );
        methods.add_method("Url", |_, this, ()| Ok(this.url.as_str().to_string()));
        methods.add_method("Path", |_, this, ()| Ok(this.url.path().to_string()));
        methods.add_method("TxtParams", |_, this, ()| {
            Ok(this.url.query().unwrap().to_string())
        });
        methods.add_method("Params", |_, this, ()| {
            let mut all_params = Vec::new();
            this.url
                .query_pairs()
                .collect::<HashMap<_, _>>()
                .iter()
                .for_each(|(param_name, _param_value)| {
                    all_params.push(param_name.to_string());
                });
            Ok(all_params)
        });
        methods.add_method("urlJoin", |_, this, new_path: String| {
            Ok(this.urljoin(new_path.as_str()))
        });
    }
}
