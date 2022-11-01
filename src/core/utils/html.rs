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

use scraper::Html;
use scraper::Selector;
use tealr::{mlu::FromToLua, TypeName};

#[derive(FromToLua, Clone, Debug, TypeName)]
pub enum Location {
    AttrValue(String),
    AttrName(String),
    TagName(String),
    Text(String),
    Comment(String),
}

/// Searching in HTML with css Selector pattern
/// you can use it for XSS Attacks or web scraping
pub fn html_search(html: &str, pattern: &str) -> String {
    let mut found = String::new();
    let document = Html::parse_document(html);
    let select = Selector::parse(pattern).unwrap();
    for node in document.select(&select) {
        found.push_str(&node.html());
    }
    found
}

/// Generate Css Selector pattern for your XSS Payload
pub fn css_selector(html: &str) -> String {
    let mut found = String::new();
    let document = Html::parse_document(html);
    document.tree.values().for_each(|node| {
        if node.as_element().is_some() {
            let element = node.as_element().unwrap();
            let mut search = format!("{}", element.name());
            element.attrs.iter().for_each(|attr| {
                if attr.1.to_string().len() > 0 {
                    search.push_str(&format!(
                        r#"[{}="{}"]"#,
                        attr.0.local,
                        attr.1.replace('\'', "\\'").replace('\"', "\\\"")
                    ));
                } else {
                    search.push_str(&format!(r#"[{}]"#, attr.0.local));
                }
            });
            if search.contains('[') {
                found.push_str(&search);
            }
        }
    });
    found
}

/// Parsing the HTML to find the location of custom Payload, the Function will return Location Enum
/// you can use it to Generate the right XSS Payload based on where the payload is reflected
pub fn html_parse(html: &str, payload: &str) -> Vec<Location> {
    let mut found: Vec<Location> = Vec::new();
    if payload.is_empty() {
        return found;
    }
    let document = Html::parse_document(html);
    document.tree.values().for_each(|node| {
        if node.is_text() {
            let text = node.as_text().unwrap();
            if text.contains(payload) {
                found.push(Location::Text(text.to_string()));
            }
        } else if node.is_element() {
            let element = node.as_element().unwrap();
            if element.name().contains(payload) {
                found.push(Location::TagName(element.name().to_string()));
            }
            element.attrs().for_each(|attr| {
                if attr.1.contains(payload) {
                    found.push(Location::AttrValue(attr.1.to_string()));
                }
                if attr.0.contains(payload) {
                    found.push(Location::AttrName(attr.0.to_string()));
                }
            });
        } else if node.is_comment() {
            let comment = node.as_comment().unwrap();
            if comment.contains(payload) {
                found.push(Location::Comment(comment.to_string()));
            }
        }
    });
    found
}
