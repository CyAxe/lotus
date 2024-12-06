// This file is part of Lotus Project, a web security scanner written in Rust based on Lua scripts.
// For details, please see https://github.com/rusty-sec/lotus/
//
// Copyright (c) 2022 - Khaled Nassar
//
// Please note that this file was originally released under the GNU General Public License as
// published by the Free Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for the specific language governing permissions
// and limitations under the License.

use console::style;
use mlua::{LuaSerdeExt, UserData};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::utils::bar::GLOBAL_PROGRESS_BAR;

#[derive(Clone, Deserialize, Serialize)]
pub struct AllReports {
    pub reports: Vec<Value>,
}

fn format_report(report: &Value) -> String {
    match report {
        Value::Object(map) => {
            let mut report_str = String::from("{");
            for (key, value) in map.iter() {
                if let Some(val_obj) = value.as_object() {
                    report_str.push_str(&format_table(key, val_obj));
                } else {
                    if key.starts_with("full_") {
                        continue;
                    }
                    report_str.push_str(&format!(
                        "\n  {} {}: {}\n",
                        style("[#]").blue(),
                        style(key).bold(),
                        style(value.to_string()).bold()
                    ));
                }
            }
            report_str.push_str("}\n\n");
            report_str
        }
        _ => "".to_string(),
    }
}

fn format_table(key: &str, val_obj: &Map<String, Value>) -> String {
    let mut table_str = format!("\n [* {}:\n", style(key).bold().green());
    for (inner_key, inner_value) in val_obj.iter() {
        if inner_key.starts_with("full_") {
            continue;
        }
        if let Some(inner_obj) = inner_value.as_object() {
            table_str.push_str(&format_table(inner_key, inner_obj));
        } else {
            let val_str = if inner_value.is_boolean() {
                if inner_value.as_bool().unwrap() {
                    "✔ true".to_owned()
                } else {
                    "✖ false".to_owned()
                }
            } else {
                inner_value.to_string()
            };
            table_str.push_str(&format!(
                "    | → {}: {}\n",
                style(inner_key).bold(),
                val_str
            ));
        }
    }
    table_str.push_str("  ]<|\n");
    table_str
}

impl UserData for AllReports {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("add", |c_lua, this, the_report: mlua::Value| {
            let the_report = c_lua.from_value(the_report).unwrap();
            {
                GLOBAL_PROGRESS_BAR.lock().unwrap().clone().unwrap().println(format_report(&the_report));
            };
            this.reports.push(the_report);
            Ok(())
        });
    }
}
