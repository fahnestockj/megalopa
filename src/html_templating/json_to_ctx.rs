use super::CtxValue;

//
pub fn parse_json_to_ctx(json_str: String) -> CtxValue {
    let mut char_iter = json_str.chars();
    
    // we need to determine if it's a valid json value
    // Either:
    // - String
    // - List
    // - Object
    // - number
    // - "true"/"false"/"null"
    // List or Object we dig into the object and recurse on sub values
    // everything else we just return

    while let Some(c) = char_iter.next() {
        match c {
            '[' => {
                // open a list
                let mut list = CtxValue::List(Box::new(vec![]));

                let mut rest_of_json_str: String = char_iter.clone().collect();
                rest_of_json_str.insert(0, c);
                let list_str = slice_until_closing_bracket(rest_of_json_str);
                let char_len = list_str.chars().count();
                // char_len - 1 for the first bracket the iter is already passed -1 for 0 indexing
                char_iter.nth(char_len - 2);

                // split on , (not inside strings) how can I split on true commas
                // I suppose valid json will only split on ], or }, but what about keys!!!!! 
                // each split can be a nested obj / other list
                
                // so we're trying to split up the contents is this it's own fn??


            }
            _ => {}
        };
    }
    todo!()
}

/// valid starting chars are '[' or '{'
/// Returns the json string opened by the starting char till it's closing char
/// IE: "{ a: 1 }, b: { c: 1 }}]" => "{ a: 1 }"
pub fn slice_until_closing_bracket(json_str: String) -> String {
    let mut json = String::new();
    let mut char_iter = json_str.chars();
    let opening_char = char_iter.clone().next().expect("Should be an opening char");
    assert!(opening_char == '[' || opening_char == '{');
    // first char will add one
    let mut braces_currently_open = 0;
    while let Some(c) = char_iter.next() {
        match c {
            '[' | '{' => {
                braces_currently_open += 1;
            }
            ']' | '}' => {
                braces_currently_open -= 1;
            }
            '\"' => {
                json.push(c);
                // we need to skip the char iter to the closing "
                // so we can avoid any brackets in the key/value
                while let Some(char) = char_iter.next() {
                    if char == '\"' {
                        break;
                    }
                    json.push(char);
                }
            }
            _ => {}
        }
        json.push(c);
        if braces_currently_open == 0 {
            break;
        }
    }
    return json;
}

#[cfg(test)]
mod tests {
    use crate::html_templating::{json_to_ctx, CtxValue};

    use super::slice_until_closing_bracket;

    #[test]
    pub fn nested_lists() {
        let json_str = "[[]],[]]".to_string();
        let res = slice_until_closing_bracket(json_str);
        assert_eq!(res, "[[]]".to_string());
    }
    #[test]
    pub fn brackets() {
        let json_str = "{[], {}}, {}]".to_string();
        let res = slice_until_closing_bracket(json_str);
        assert_eq!(res, "{[], {}}".to_string());
    }
    #[test]
    pub fn brackets_in_key() {
        let json_str = "{ \"}}}key\": 1 }".to_string();
        let res = slice_until_closing_bracket(json_str.clone());
        assert_eq!(res, json_str);
    }
    #[test]
    pub fn couple_keys() {
        let json_str = "{ \"k1\": \"value\", \"k2\": {} }".to_string();
        let res = slice_until_closing_bracket(json_str.clone());
        assert_eq!(res, json_str);
    }
}
