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


use log::{debug, error};
use mlua::Lua;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

pub fn report_script(report_code: &str) {
    debug!("Running the Report Script");
    let lua_vm = Lua::new();
    lua_report_func(&lua_vm);
    match lua_vm.load(report_code).exec() {
        Ok(_done) => {}
        Err(err) => {
            println!("CODE: {}", report_code);
            println!("ERR {:?}", err);
            error!("Lua Report Error: {}", err);
        }
    }
}

fn lua_report_func(vm: &Lua) -> &Lua {
    vm.globals()
        .set(
            "to_json",
            vm.create_function(|_, data: mlua::Table| {
                let mut test_report: HashMap<String, mlua::Value> = HashMap::new();
                data.pairs::<String, mlua::Value>().for_each(|out_report| {
                    let current_out = out_report.clone();
                    test_report.insert(current_out.unwrap().0, out_report.unwrap().1);
                });
                let results = serde_json::to_string(&test_report).unwrap();
                Ok(results)
            })
            .unwrap(),
        )
        .unwrap();

    vm.globals()
        .set(
            "save_file",
            vm.create_function(|_, (data, report_path): (String, String)| {
                save_file(&report_path, &data);
                Ok(())
            })
            .unwrap(),
        )
        .unwrap();
    vm
}

fn save_file(file_path: &str, results: &str) {
    OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Could not open file")
        .write_all(format!("{}\n", results).as_str().as_bytes())
        .expect("Could not write to file");
}
