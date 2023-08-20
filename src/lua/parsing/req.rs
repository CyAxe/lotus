use crate::lua::network::http::HttpResponse;
use mlua::Lua;
use std::borrow::Cow;

pub fn show_response<'lua>(
    _: &'lua Lua,
    res: HttpResponse,
) -> Result<Cow<'static, str>, mlua::Error> {
    let headers_str = {
        let mut headers_str = String::new();
        res.headers.iter().for_each(|(headername, headervalue)| {
            headers_str.push_str(&format!("\n{headername}: {headervalue}"));
        });
        headers_str
    };
    let body = &res.body;
    let version = res.version;
    let reason = res.reason;
    Ok(Cow::from(format!(
        r#"{version} {reason}{headers_str}


{body}
        "#
    )))
}
