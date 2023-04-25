use crate::lua::network::http::HttpResponse;
use std::borrow::Cow;

pub fn show_response(res: &HttpResponse) -> Cow<'static, str> {
    let headers_str = {
        let mut headers_str = String::new();
        res.headers.iter().for_each(|(headername,headervalue)|{
            headers_str.push_str(&format!("\n{headername}: {headervalue}"));
        });
        headers_str
    };
    let body = &res.body;
    let status = res.status;
    Cow::from(format!(r#"HTTP/1.1 {status}{headers_str}


{body}
        "#))
}
