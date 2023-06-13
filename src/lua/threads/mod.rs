pub mod runner;
use futures::{executor::block_on, stream, StreamExt};
use mlua::UserData;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct LuaThreader {
    pub stop: Arc<Mutex<bool>>,
}

#[derive(Clone)]
pub struct ParamScan {
    pub finds: Arc<Mutex<bool>>,
    pub accept_nil: Arc<Mutex<bool>>,
}

impl ParamScan {
    pub async fn stop_scan(&mut self) {
        *self.finds.lock().await = true;
    }
}

impl UserData for ParamScan {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("start_scan", |_, this, ()| async move {
            *this.finds.lock().await = false;
            Ok(())
        });
        methods.add_async_method("accept_nil", |_, this, accept_nil: bool| async move {
            *this.accept_nil.lock().await = accept_nil;
            Ok(())
        });
        methods.add_method("is_accept_nil", |_, this, ()| {
            Ok(*block_on(this.accept_nil.lock()))
        });
        methods.add_async_method(
            "add_scan",
            |_,
             this,
             (target_param, iter_payload, target_func, callback_function, workers): (
                mlua::Value,
                Vec<mlua::Value>,
                mlua::Function,
                mlua::Function,
                usize,
            )| async move {
                let target_func = Arc::new(target_func);
                let callback_function = Arc::new(callback_function);
                stream::iter(iter_payload)
                    .map(move |target_table| {
                        let mut stop_scan = false;
                        let target_func = Arc::clone(&target_func);
                        let target_param = target_param.clone();
                        let callback_function = Arc::clone(&callback_function);
                        let accept_nil = Arc::clone(&this.accept_nil);
                        if *block_on(this.finds.lock()) {
                            stop_scan = true;
                        }
                        async move {
                            if !stop_scan {
                                let target_param = target_param.clone();
                                let caller = target_func
                                    .call_async::<_, mlua::Value>((
                                        target_param,
                                        target_table,
                                    ))
                                    .await
                                    .unwrap();
                                let is_nil = { caller == mlua::Nil };
                                if is_nil {
                                    if *accept_nil.lock().await {
                                        callback_function
                                            .call_async::<_, bool>(caller)
                                            .await
                                            .unwrap();
                                    }
                                } else {
                                    callback_function
                                        .call_async::<_, bool>(caller)
                                        .await
                                        .unwrap();
                                }
                            }
                        }
                    })
                    .buffer_unordered(workers)
                    .collect::<Vec<_>>()
                    .await;
                Ok(())
            },
        );
        methods.add_method_mut("stop_scan", |_, this, ()| {
            block_on(this.stop_scan());
            Ok(())
        });
        methods.add_method("is_stop", |_, this, ()| Ok(*block_on(this.finds.lock())));
    }
}

impl UserData for LuaThreader {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("run_scan", |_, this, (iter_data, target_func,workers): (Vec<mlua::Value>, mlua::Function, usize)| async move {
            let target_func = Arc::new(target_func);
            stream::iter(iter_data)
                .map(move |target_table| {
                    let target_func = Arc::clone(&target_func);
                    let stop_scan: bool = *block_on(this.stop.lock());
                    async move {
                        if stop_scan {
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
        methods.add_async_method("is_stop", |_, this, ()| async move {
            Ok(*this.stop.lock().await)
        });
    }
}
