macro_rules! set_global_function {
    ($lua:expr, $name:expr, $func:expr) => {
        $lua.globals().set($name, $func).unwrap();
    };
}

pub fn tester() {
    let lua = tealr::mlu::mlua::Lua::new();
    let _ = lua.load("print(\"test\")").exec().unwrap();

}
