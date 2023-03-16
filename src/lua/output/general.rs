use mlua::UserData;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeneralReport {
    pub name: Option<String>,
    pub description: Option<String>,
    pub target: Option<String>,
    pub risk: Option<String>,
    pub matchers: Vec<serde_json::Value>,
}

impl UserData for GeneralReport {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("setRisk", |_, this, risk: String| {
            this.risk = Some(risk);
            Ok(())
        });
        methods.add_method_mut("setName", |_, this, name: String| {
            this.name = Some(name);
            Ok(())
        });
        methods.add_method_mut("setTarget", |_, this, target: String| {
            this.target = Some(target);
            Ok(())
        });

        methods.add_method_mut("setDescription", |_, this, description: String| {
            this.description = Some(description);
            Ok(())
        });
        methods.add_method_mut("setMatchers", |_, this, matching_data: mlua::Value| {
            let matching_data_json = serde_json::to_value(&matching_data).unwrap();
            this.matchers.push(matching_data_json);
            Ok(())
        });
    }
}

impl GeneralReport {
    pub fn init() -> Self {
        Self {
            name: None,
            target: None,
            description: None,
            risk: None,
            matchers: vec![],
        }
    }
}
