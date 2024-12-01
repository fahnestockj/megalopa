mod spec_tests;
use std::collections::HashMap;
mod escape_html;
mod json_to_ctx;
// A mustache compliant templating engine 🚀

struct TemplateEngine;

trait OneoffRender {
    fn oneoff_render(&self, template_string: String, context: HashMap<&str, CtxValue>) -> String;
}

impl OneoffRender for TemplateEngine {
    fn oneoff_render(&self, template_string: String, context: HashMap<&str, CtxValue>) -> String {
        mustachify(template_string, context)
    }
}

#[derive(Clone, Debug)]
pub enum CtxValue {
    String(String),
    Boolean(bool),
    Number(i8),
    HashMap(Box<HashMap<String, CtxValue>>),
    List(Box<Vec<CtxValue>>),
}

pub fn mustachify(template_string: String, context: HashMap<&str, CtxValue>) -> String {
    let mut result = String::new();
    let mut char_iter = template_string.chars();

    while let Some(char) = char_iter.next() {
        let mut sub_iter = char_iter.clone();
        if char == '{' && sub_iter.next().is_some_and(|next_char| next_char == '{') {
            // check for three in a rowwww and ampersand
            let is_triple_stache = sub_iter.clone().next().is_some_and(|char| char == '{');
            let is_html_escaped =
                !(is_triple_stache || sub_iter.clone().next().is_some_and(|char| char == '&'));
            // closing tags?
            let closing_pattern = if is_triple_stache { "}}}" } else { "}}" };
            // rest_of_line after the first {
            let rest_of_line: String = char_iter.clone().collect();
            if let Some(closing_stache_byte_idx) = rest_of_line.find(closing_pattern) {
                // if we have found this then char_iter is on the first { sub_iter is on the second {
                let skip = if is_triple_stache { 3 } else { 2 };
                // rest_of_line includes a starting { or {{ so - (skip-1)
                let char_len_until_end_of_closing_brackets =
                    rest_of_line[..closing_stache_byte_idx].chars().count() + skip;
                let mut content_in_stache: String = rest_of_line[..closing_stache_byte_idx]
                    .chars()
                    .skip(skip - 1)
                    .collect();
                if content_in_stache
                    .chars()
                    .next()
                    .is_some_and(|char| char == '&')
                {
                    content_in_stache = content_in_stache
                        .strip_prefix("&")
                        .expect("should be there")
                        .to_string();
                }
                // here's where we start thinking about all the different stache statements
                if content_in_stache.starts_with("#") {
                    // Section!
                } else if content_in_stache.starts_with("^") {
                    // Inverted Section!
                } else if content_in_stache.starts_with(">") {
                    // Partial!
                } else if content_in_stache.starts_with("!") {
                    // Comment!
                } else if content_in_stache.starts_with("$") {
                    // Block!
                } else {
                    // regular variable
                    let key = content_in_stache.trim();
                    let variable_value = match context.get(key) {
                        Some(val) => val.clone(),
                        None => CtxValue::String("".to_string()),
                    };
                    // makes sense for num and string but boolean?? this is smelly
                    let mut string_value = match variable_value {
                        CtxValue::Boolean(_) => "".to_string(),
                        CtxValue::Number(n) => n.to_string(),
                        CtxValue::String(s) => s,
                        _ => { todo!()}
                    };
                    if is_html_escaped {
                        string_value = escape_html::escape_html(string_value);
                    }
                    result.push_str(&string_value);
                    char_iter.nth(char_len_until_end_of_closing_brackets - 1);
                }
            } else {
                result.push(char);
            }
        } else {
            result.push(char);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn empty_var() {
        let hash: HashMap<&str, CtxValue> = HashMap::new();
        let stache = mustachify("some text {{ stache }} some text".to_string(), hash);
        assert_eq!(stache, "some text  some text".to_string())
    }

    #[test]
    pub fn with_var() {
        let mut hash: HashMap<&str, CtxValue> = HashMap::new();
        hash.insert("stache", CtxValue::String("world".to_string()));
        let res = mustachify("hello {{ stache }} \n".to_string(), hash);
        assert_eq!(res, "hello world \n".to_string())
    }
}
