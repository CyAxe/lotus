use mlua::UserData;
use serde::{Deserialize, Serialize};

#[derive(Clone,Deserialize, Serialize)]
pub struct OutReport {
    pub risk: Option<String>,        // info, low, Medium, High, Ciritcal
    pub name: Option<String>,        // name of your bug
    pub description: Option<String>, // talk about it
    pub url: Option<String>,         // the effected url
    pub param: Option<String>,       // the effected parameter`
    pub attack: Option<String>,      // the payload
    pub evidence: Option<String>,    // matching payload
                                     // TODO: Request: RequestOpts {url, method, timeout, etc...}
}

#[derive(Clone)]
pub struct AllReports {
    pub reports: Vec<OutReport>,
}

impl UserData for AllReports {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("addReport", |_, this, the_report: OutReport| {
            this.reports.push(the_report);
            Ok(())
        });
    }
}

impl UserData for OutReport {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("setRisk", |_, this, risk: String| {
            this.risk = Some(risk);
            Ok(())
        });
        methods.add_method_mut("setName", |_, this, name: String| {
            this.name = Some(name);
            Ok(())
        });
        methods.add_method_mut("setUrl", |_, this, url: String| {
            this.url = Some(url);
            Ok(())
        });

        methods.add_method_mut("setDescription", |_, this, description: String| {
            this.description = Some(description);
            Ok(())
        });

        methods.add_method_mut("setParam", |_, this, param: String| {
            this.param = Some(param);
            Ok(())
        });

        methods.add_method_mut("setAttack", |_, this, attack: String| {
            this.attack = Some(attack);
            Ok(())
        });

        methods.add_method_mut("setEvidence", |_, this, evidence: String| {
            this.evidence = Some(evidence);
            Ok(())
        });
    }
}

impl OutReport {
    pub fn init() -> OutReport {
        OutReport {
            risk: None,
            name: None,
            description: None,
            url: None,
            param: None,
            attack: None,
            evidence: None,
        }
    }

}
