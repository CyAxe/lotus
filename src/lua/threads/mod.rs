use futures::{stream, StreamExt};
use mlua::UserData;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct LuaThreader {
    pub stop: Arc<Mutex<bool>>,
}

impl UserData for LuaThreader {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("run_scan", |_, this, (iter_data, target_func,workers): (Vec<mlua::Value>, mlua::Function, usize)| async move {
            let target_func = Arc::new(target_func);
            stream::iter(iter_data)
                .map(move |target_table| {
                    let target_func = Arc::clone(&target_func);
                    let stop_scan: bool;
                    if *this.stop.lock().unwrap() == true {
                        stop_scan = true;
                    } else {
                        stop_scan = false;
                    }
                    async move {
                        if stop_scan == true {
                            // Ignore
                        } else {
                            target_func.call_async::<_, mlua::Value>(target_table).await.unwrap();
                        }
                    }
                })
                .buffer_unordered(workers)
                .collect::<Vec<_>>().await;
            Ok(())
        });
        methods.add_method_mut("stop_scan", |_, this, ()| {
            this.stop = Arc::new(Mutex::new(true));
            Ok(())
        });
        methods.add_method("is_stop", |_, this, ()| Ok(*this.stop.lock().unwrap()));
    }
}
