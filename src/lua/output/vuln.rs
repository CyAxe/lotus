/*
 * This file is part of Lotus Project, an Web Security Scanner written in Rust based on Lua Scripts
 * For details, please see https://github.com/rusty-sec/lotus/
 *
 * Copyright (c) 2022 - Khaled Nassar
 *
 * Please note that this file was originally released under the
 * GNU General Public License as published by the Free Software Foundation;
 * either version 2 of the License, or (at your option) any later version.
 *
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::lua::output::cve::CveReport;
use mlua::UserData;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "report_type")]
pub enum LotusReport {
    CVE(CveReport),
    VULN(OutReport),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct OutReport {
    pub risk: Option<String>,        // info, low, Medium, High, Ciritcal
    pub name: Option<String>,        // name of your bug
    pub description: Option<String>, // talk about it
    pub url: Option<String>,         // the effected url
    pub param: Option<String>,       // the effected parameter`
    pub attack: Option<String>,      // the payload
    pub evidence: Option<String>,    // matching payload search pattern
                                     // TODO: Request: RequestOpts {url, method, timeout, etc...}
}

#[derive(Clone)]
pub struct AllReports {
    pub reports: Vec<LotusReport>,
}

impl UserData for AllReports {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("addVulnReport", |_, this, the_report: OutReport| {
            this.reports.push(LotusReport::VULN(the_report));
            Ok(())
        });
        methods.add_method_mut("addCveReport", |_, this, the_report: CveReport| {
            this.reports.push(LotusReport::CVE(the_report));
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
