#[cfg(test)]
mod tests {
	use crate::html_templating::{TemplateEngine, OneoffRender, CtxValue};
	
// Set Delimiter tags are used to change the tag delimiters for all content
// following the tag in the current compilation unit.
// 
// The tag's content MUST be any two non-whitespace sequences (separated by
// whitespace) EXCEPT an equals sign ('=') followed by the current closing
// delimiter.
// 
// Set Delimiter tags SHOULD be treated as standalone when appropriate.

/// The equals sign (used on both sides) should permit delimiter changes.
#[test]
pub fn pair_behavior () {
	let template = "{{=<% %>=}}(<%text%>)".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("text",CtxValue::String("Hey!".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("(Hey!)");
	assert_eq!(result, expected)
}

/// Characters with special meaning regexen should be valid delimiters.
#[test]
pub fn special_characters () {
	let template = "({{=[ ]=}}[text])".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("text",CtxValue::String("It worked!".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("(It worked!)");
	assert_eq!(result, expected)
}

/// Delimiters set outside sections should persist.
#[test]
pub fn sections () {
	let template = "[\n{{#section}}\n  {{data}}\n  |data|\n{{/section}}\n\n{{= | | =}}\n|#section|\n  {{data}}\n  |data|\n|/section|\n]\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("section",CtxValue::Boolean(true));
	ctx.insert("data",CtxValue::String("I got interpolated.".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("[\n  I got interpolated.\n  |data|\n\n  {{data}}\n  I got interpolated.\n]\n");
	assert_eq!(result, expected)
}

/// Delimiters set outside inverted sections should persist.
#[test]
pub fn inverted_sections () {
	let template = "[\n{{^section}}\n  {{data}}\n  |data|\n{{/section}}\n\n{{= | | =}}\n|^section|\n  {{data}}\n  |data|\n|/section|\n]\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("section",CtxValue::Boolean(false));
	ctx.insert("data",CtxValue::String("I got interpolated.".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("[\n  I got interpolated.\n  |data|\n\n  {{data}}\n  I got interpolated.\n]\n");
	assert_eq!(result, expected)
}

/// Delimiters set in a parent template should not affect a partial.
#[test]
pub fn partial_inheritence () {
	let template = "[ {{>include}} ]\n{{= | | =}}\n[ |>include| ]\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("value",CtxValue::String("yes".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("[ .yes. ]\n[ .yes. ]\n");
	assert_eq!(result, expected)
}

/// Delimiters set in a partial should not affect the parent template.
#[test]
pub fn postpartial_behavior () {
	let template = "[ {{>include}} ]\n[ .{{value}}.  .|value|. ]\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("value",CtxValue::String("yes".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("[ .yes.  .yes. ]\n[ .yes.  .|value|. ]\n");
	assert_eq!(result, expected)
}

/// Surrounding whitespace should be left untouched.
#[test]
pub fn surrounding_whitespace () {
	let template = "| {{=@ @=}} |".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("|  |");
	assert_eq!(result, expected)
}

/// Whitespace should be left untouched.
#[test]
pub fn outlying_whitespace_inline () {
	let template = " | {{=@ @=}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from(" | \n");
	assert_eq!(result, expected)
}

/// Standalone lines should be removed from the template.
#[test]
pub fn standalone_tag () {
	let template = "Begin.\n{{=@ @=}}\nEnd.\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("Begin.\nEnd.\n");
	assert_eq!(result, expected)
}

/// Indented standalone lines should be removed from the template.
#[test]
pub fn indented_standalone_tag () {
	let template = "Begin.\n  {{=@ @=}}\nEnd.\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("Begin.\nEnd.\n");
	assert_eq!(result, expected)
}

/// "\r\n" should be considered a newline for standalone tags.
#[test]
pub fn standalone_line_endings () {
	let template = "|\r\n{{= @ @ =}}\r\n|".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("|\r\n|");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to precede them.
#[test]
pub fn standalone_without_previous_line () {
	let template = "  {{=@ @=}}\n=".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("=");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to follow them.
#[test]
pub fn standalone_without_newline () {
	let template = "=\n  {{=@ @=}}".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("=\n");
	assert_eq!(result, expected)
}

/// Superfluous in-tag whitespace should be ignored.
#[test]
pub fn pair_with_padding () {
	let template = "|{{= @   @ =}}|".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("||");
	assert_eq!(result, expected)
}
}