#[cfg(test)]
mod tests {
	use crate::html_templating::{TemplateEngine, OneoffRender, CtxValue};
	

/// The greater-than operator should expand to the named partial.
#[test]
pub fn basic_behavior () {
	let template = "\"{{>text}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"from partial\"");
	assert_eq!(result, expected)
}

/// The empty string should be used when the named partial is not found.
#[test]
pub fn failed_lookup () {
	let template = "\"{{>text}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"\"");
	assert_eq!(result, expected)
}

/// The greater-than operator should operate within the current context.
#[test]
pub fn context () {
	let template = "\"{{>partial}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert(".text",CtxValue::String("content".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"*content*\"");
	assert_eq!(result, expected)
}

/// The greater-than operator should properly recurse.
#[test]
pub fn recursion () {
	let template = "{{>node}}".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert(".content",CtxValue::String("X".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("X<Y<>>");
	assert_eq!(result, expected)
}

/// The greater-than operator should work from within partials.
#[test]
pub fn nested () {
	let template = "{{>outer}}".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert(".a",CtxValue::String("hello".to_string()));
	ctx.insert(".b",CtxValue::String("world".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("*hello world!*");
	assert_eq!(result, expected)
}

/// The greater-than operator should not alter surrounding whitespace.
#[test]
pub fn surrounding_whitespace () {
	let template = "| {{>partial}} |".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("| \t|\t |");
	assert_eq!(result, expected)
}

/// Whitespace should be left untouched.
#[test]
pub fn inline_indentation () {
	let template = "  {{data}}  {{> partial}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert(".data",CtxValue::String("|".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("  |  >\n>\n");
	assert_eq!(result, expected)
}

/// "\r\n" should be considered a newline for standalone tags.
#[test]
pub fn standalone_line_endings () {
	let template = "|\r\n{{>partial}}\r\n|".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("|\r\n>|");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to precede them.
#[test]
pub fn standalone_without_previous_line () {
	let template = "  {{>partial}}\n>".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("  >\n  >>");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to follow them.
#[test]
pub fn standalone_without_newline () {
	let template = ">\n  {{>partial}}".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from(">\n  >\n  >");
	assert_eq!(result, expected)
}

/// Each line of the partial should be indented before rendering.
#[test]
pub fn standalone_indentation () {
	let template = "\\\n {{>partial}}\n/\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert(".content",CtxValue::String("<\n->".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\\\n |\n <\n->\n |\n/\n");
	assert_eq!(result, expected)
}

/// Superfluous in-tag whitespace should be ignored.
#[test]
pub fn padding_whitespace () {
	let template = "|{{> partial }}|".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert(".boolean",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("|[]|");
	assert_eq!(result, expected)
}
}