use crate::{
    BAR,
    LuaRunTime,
    lua::{
        loader::is_match,
        parsing::{
            text::ResponseMatcher,
            html::{html_search, html_parse, css_selector}
        },
        threads::{LuaThreader, ParamScan},
        output::{
        vuln::OutReport,
        cve::CveReport
        }
    }
};
use std::sync::{Arc,Mutex};
use mlua::ExternalError;
use console::Style;


pub trait UtilsEXT {
    fn add_threadsfunc(&self);
    fn add_matchingfunc(&self);
    fn add_printfunc(&self);
}

impl UtilsEXT for LuaRunTime<'_> {
    fn add_printfunc(&self) {
        self.lua
            .globals()
            .set(
                "print_cve_report",
                self.lua
                    .create_function(move |_, the_report: CveReport| {
                        let good_msg = format!("[{}]", Style::new().green().apply_to("+"));
                        let info_msg = format!("[{}]", Style::new().blue().apply_to("#"));
                        let report_msg = format!(
                            "
{GOOD} {NAME} on: {URL}
{INFO} SCAN TYPE: CVE
{INFO} Description: {Description}
{INFO} Risk: {RISK}
{INFO} Matching Pattern: {MATCHING}
#--------------------------------------------------#

                                     ",
                            GOOD = good_msg,
                            INFO = info_msg,
                            NAME = the_report.name.unwrap(),
                            URL = the_report.url.unwrap(),
                            Description = the_report.description.unwrap(),
                            RISK = the_report.risk.unwrap(),
                            MATCHING = format!("{:?}", the_report.matchers),
                        );
                        {
                            BAR.lock().unwrap().println(report_msg)
                        };
                        Ok(())
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "print_vuln_report",
                self.lua
                    .create_function(move |_, the_report: OutReport| {
                        let good_msg = format!("[{}]", Style::new().green().apply_to("+"));
                        let info_msg = format!("[{}]", Style::new().blue().apply_to("#"));
                        let report_msg = format!(
                            "
{GOOD} {NAME} on: {URL}
{INFO} SCAN TYPE: VULN
{INFO} Description: {Description}
{INFO} Vulnerable Parameter: {PARAM}
{INFO} Risk: {RISK}
{INFO} Used Payload: {ATTACK}
{INFO} Matching Pattern: {MATCHING}
#--------------------------------------------------#

                                     ",
                            GOOD = good_msg,
                            INFO = info_msg,
                            NAME = the_report.name.unwrap(),
                            URL = the_report.url.unwrap(),
                            Description = the_report.description.unwrap(),
                            PARAM = the_report.param.unwrap(),
                            RISK = the_report.risk.unwrap(),
                            ATTACK = the_report.attack.unwrap(),
                            MATCHING = format!(
                                "{}",
                                Style::new().on_red().apply_to(the_report.evidence.unwrap())
                            ),
                        );
                        {
                            BAR.lock().unwrap().println(report_msg)
                        };
                        Ok(())
                    })
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "println",
                self.lua
                    .create_function(move |_, msg: String| {
                        {
                            BAR.lock().unwrap().println(&msg)
                        };
                        Ok(())
                    })
                    .unwrap(),
            )
            .unwrap();

    }
    fn add_matchingfunc(&self) {
        self.lua
            .globals()
            .set(
                "is_match",
                self.lua
                    .create_function(|_, (pattern, text): (String, String)| {
                        let try_match = is_match(pattern, text);
                        if try_match.is_err() {
                            Err(try_match.unwrap_err().to_lua_err())
                        } else {
                            Ok(try_match.unwrap())
                        }
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "generate_css_selector",
                self.lua
                    .create_function(|_, payload: String| Ok(css_selector(&payload)))
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "html_parse",
                self.lua
                    .create_function(|_, (html, payload): (String, String)| {
                        Ok(html_parse(&html, &payload))
                    })
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "html_search",
                self.lua
                    .create_function(|_, (html, pattern): (String, String)| {
                        Ok(html_search(&html, &pattern))
                    })
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set("ResponseMatcher", ResponseMatcher {})
            .unwrap();

        self.lua
            .globals()
            .set(
                "str_startswith",
                self.lua
                    .create_function(|_, (str_one, str_two): (String, String)| {
                        Ok(str_one.starts_with(&str_two))
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "str_contains",
                self.lua
                    .create_function(|_, (str_one, str_two): (String, String)| {
                        Ok(str_one.contains(&str_two))
                    })
                    .unwrap(),
            )
            .unwrap();
    }

    fn add_threadsfunc(&self) {
        // ProgressBar
        self.lua
            .globals()
            .set(
                "ParamScan",
                ParamScan {
                    finds: Arc::new(Mutex::new(false)),
                    accept_nil: Arc::new(Mutex::new(false)),
                },
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "LuaThreader",
                LuaThreader {
                    stop: Arc::new(Mutex::new(false)),
                },
            )
            .unwrap();
    }
}
