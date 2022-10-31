use mlua::UserData;
use std::collections::HashMap;
use url::Url;

#[derive(Clone)]
pub struct HttpMessage {
    pub url: Url,
}

impl UserData for HttpMessage {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("setParam", |_, this, (param, payload): (String, String)| {
            Ok(this.set_urlvalue(&param, &payload))
        });
        methods.add_method(
            "setAllParams",
            |_, this, (payload, remove_content): (String, bool)| {
                Ok(this.change_urlquery(payload, remove_content))
            },
        );
        methods.add_method("getParams", |_, this, ()| {
            Ok(this.url.query().unwrap().to_string())
        });
        methods.add_method("urlJoin", |_, this, new_path: String|{
            Ok(this.urljoin(new_path))
        });
    }
}

impl HttpMessage {
    pub fn change_urlquery(
        &self,
        payload: String,
        remove_content: bool,
    ) -> HashMap<String, String> {
        let url = self.url.clone();
        let mut scan_params = HashMap::new();
        let mut result: HashMap<String, String> = HashMap::new();
        let mut param_list = Vec::new();

        url.query_pairs()
            .collect::<HashMap<_, _>>()
            .iter()
            .for_each(|(key, value)| {
                scan_params.insert(key.to_string(), value.to_string());
                param_list.push(key.to_string());
            });

        scan_params.iter().for_each(|(key, value)| {
            payload.split('\n').into_iter().for_each(|payload| {
                let mut new_params = scan_params.clone();
                if remove_content {
                    new_params.insert(key.to_string(), payload.to_string());
                } else {
                    new_params.insert(key.to_string(), value.as_str().to_owned() + payload);
                }
                let mut new_url = url.clone();
                new_url.query_pairs_mut().clear();

                new_url.query_pairs_mut().extend_pairs(&new_params);

                result.insert(key.to_string(), new_url.as_str().to_string());
            });
        });
        result
    }
    pub fn set_urlvalue(&self, param: &str, payload: &str) -> String {
        let mut url = self.url.clone();
        let mut final_params = HashMap::new();
        url.query_pairs()
            .into_iter()
            .collect::<HashMap<_, _>>()
            .iter()
            .for_each(|(k, v)| {
                if k == param {
                    final_params.insert(k.to_string(), format!("{}{}", v, payload));
                } else {
                    final_params.insert(k.to_string(), v.to_string());
                }
            });
        url.query_pairs_mut().clear();
        url.query_pairs_mut().extend_pairs(final_params);
        url.as_str().to_string()
    }

    pub fn urljoin(&self, path: String) -> String {
        self.url.join(&path).unwrap().as_str().to_string()
    }
}
