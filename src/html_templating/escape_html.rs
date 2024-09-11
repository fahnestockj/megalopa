pub fn escape_html(string: String) -> String {
    let mut escaped_string = String::new();
    string.chars().for_each(|char| match char {
        '<' => escaped_string.push_str("&lt"),
        '>' => escaped_string.push_str("&gt"),
        '&' => escaped_string.push_str("&amp"),
        '"' => escaped_string.push_str("&quot"),
        '\'' => escaped_string.push_str("&#39"),
        _ => escaped_string.push(char),
    });
    return escaped_string;
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn escaping_html() {
    let escaped_html = escape_html("hello & world <script>alert(\"Uh oh\")</script>".to_string());
    assert_eq!(escaped_html, "hello &amp world &ltscript&gtalert(&quotUh oh&quot)&lt/script&gt".to_string())
  }

}
