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
