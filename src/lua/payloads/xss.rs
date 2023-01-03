mod xsspayloads;
use crate::lua::parsing::html::{css_selector, Location};
use fancy_regex::Regex;
use tealr::{mlu::FromToLua, TypeName};
use xsspayloads::{get_attr, get_htmltags, get_jscmd, get_jsvalue};

fn match_qoutes(d: &str, s: &str) -> bool {
    let re = Regex::new(&format!(r#"\'.*{}.*\'"#, s)).unwrap();
    re.is_match(d).unwrap_or(false)
}

fn match_double_qoutes(d: &str, s: &str) -> bool {
    let re = Regex::new(&format!(r#"\".*{}.*\""#, s)).unwrap();
    re.is_match(d).unwrap_or(false)
}

#[derive(Debug, Clone)]
pub struct XssPayloads {
    pub js_cmd: Vec<String>,
    pub js_value: Vec<String>,
    pub attr: Vec<String>,
    pub html_tags: Vec<String>,
}

#[derive(FromToLua, Clone, Debug, TypeName)]
pub struct OrderPayload {
    pub search: String,
    pub payload: String,
}

pub struct PayloadGen {
    pub location: Location,
    pub response: String,
    pub payload: String,
    pub payloads: XssPayloads,
}

impl PayloadGen {
    pub fn new(response: String, location: Location, payload: String) -> Self {
        PayloadGen {
            location,
            response,
            payloads: XssPayloads {
                js_cmd: get_jscmd(),
                js_value: get_jsvalue(),
                attr: get_attr(),
                html_tags: get_htmltags(),
            },
            payload,
        }
    }

    pub fn txt_payloads(&self, before_payload: &str) -> Vec<OrderPayload> {
        let mut payloads = vec![];
        self.payloads.html_tags.iter().for_each(|tag| {
            if !tag.contains("$JS_FUNC$") && !tag.contains("$JS_CMD$") {
                let payload = format!("{}{}", before_payload, tag);
                let search = css_selector(&payload);
                payloads.push(OrderPayload { payload, search });
            } else {
                self.payloads.js_cmd.iter().for_each(|cmd| {
                    self.payloads.js_value.iter().for_each(|value| {
                        let payload = format!(
                            "{}{}",
                            before_payload,
                            tag.replace("$JS_FUNC$", cmd).replace("$JS_CMD$", value)
                        );
                        let search = css_selector(&payload);
                        payloads.push(OrderPayload { payload, search });
                    });
                });
            }
        });
        payloads
    }

    pub fn tagname_payloads(&self) -> Vec<OrderPayload> {
        let mut payloads = vec![];
        self.payloads.attr.iter().for_each(|attr| {
            self.payloads.js_cmd.iter().for_each(|cmd| {
                self.payloads.js_value.iter().for_each(|value| {
                    for space in 1..5 {
                        payloads.push(OrderPayload {
                            payload: format!(
                                "{}{} {}={}({}){}",
                                "v".repeat(space),
                                " ".repeat(space),
                                attr,
                                cmd,
                                value,
                                " ".repeat(space)
                            ),
                            search: format!(r#"*[{}="{}({})"]"#, attr, cmd, value),
                        });

                        payloads.push(OrderPayload {
                            payload: format!(
                                "{}{} {}={}`{}`{}",
                                "v".repeat(space),
                                " ".repeat(space),
                                attr,
                                cmd,
                                value,
                                " ".repeat(space)
                            ),
                            search: format!("*[{}=\"{}`{}`\"]", attr, cmd, value),
                        });
                    }
                });
            });
        });
        payloads
    }

    pub fn attrvalue_payloads(&self, qoutes: &str) -> Vec<OrderPayload> {
        let mut payloads = Vec::new();
        let payloads_with_attr = {
            let mut new_payloads = vec![];
            self.payloads.js_cmd.iter().for_each(|y| {
                self.payloads.js_value.iter().for_each(|z| {
                    new_payloads.push(format!("{}({})", y, z));
                })
            });
            new_payloads
        };
        payloads_with_attr.iter().for_each(|js_cmd| {
            self.payloads.attr.iter().for_each(|attr_param| {
                for i in 0..5 {
                    payloads.push(OrderPayload {
                        payload: {
                            format!(
                                "{}{}={} {}",
                                qoutes.repeat(i),
                                attr_param,
                                js_cmd,
                                " vd ".repeat(i)
                            )
                        },
                        search: format!(r#"*[{}="{}"]"#, attr_param, js_cmd),
                    });
                }
            });
        });
        payloads
    }

    pub fn attrname_payloads(&self) -> Vec<OrderPayload> {
        let mut payloads: Vec<OrderPayload> = Vec::new();
        self.payloads.attr.iter().for_each(|attr| {
            self.payloads.js_cmd.iter().for_each(|js_cmd| {
                self.payloads.js_value.iter().for_each(|js_value| {
                    for i in 0..5 {
                        payloads.push(OrderPayload {
                            payload: format!(
                                "{}{}={}({}) {}",
                                "v ".repeat(i),
                                attr,
                                js_cmd,
                                js_value,
                                "v ".repeat(i)
                            ),
                            search: format!("*[{}=\"{}({})\"]", attr, js_cmd, js_value),
                        });
                    }
                });
            });
        });
        payloads
    }

    pub fn analyze(&self) -> Vec<OrderPayload> {
        match self.location {
            Location::Text(ref _txt) => self.txt_payloads(""),
            Location::TagName(ref _txt) => self.tagname_payloads(),
            Location::AttrName(ref _txt) => self.attrname_payloads(),
            Location::AttrValue(ref attr_value) => {
                let double = match_double_qoutes(self.response.as_str(), attr_value.as_str());
                let single = match_qoutes(self.response.as_str(), attr_value.as_str());
                if double {
                    self.attrvalue_payloads("\"")
                } else if single {
                    self.attrvalue_payloads("'")
                } else {
                    self.attrvalue_payloads("v ")
                }
            }
            Location::Comment(ref _txt) => self.txt_payloads("--> --> -->"),
        }
    }
}
