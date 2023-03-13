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

use interactsh_rs::client::{builder::ClientBuilder, registered::RegisteredClient};
use mlua::UserData;

impl UserData for OAst {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("host", |_ctx, this, ()| async move { Ok(this.host) });

        methods.add_async_method("poll", |_, this, ()| async move {
            let logs = this.controle.poll().await.unwrap();
            logs.iter().for_each(|x| println!("ff {:?}", x));
            Ok(())
        });
    }
}

#[derive(Clone)]
pub struct OAstLog {
    pub host: String,
    pub unique_id: String,
    pub raw_request: String,
}

#[derive(Clone)]
pub struct OAst {
    pub controle: RegisteredClient,
    pub host: String,
    pub data: Vec<OAstLog>,
}

impl OAst {
    pub async fn init() -> OAst {
        let controler = ClientBuilder::default()
            .parse_logs(true)
            .verify_ssl(false)
            .build()
            .unwrap()
            .register()
            .await
            .unwrap();
        OAst {
            controle: controler.clone(),
            host: controler.get_interaction_url(),
            data: Vec::new(),
        }
    }
}
