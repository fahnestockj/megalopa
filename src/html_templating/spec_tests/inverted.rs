#[cfg(test)]
mod tests {
	use crate::html_templating::{TemplateEngine, OneoffRender, CtxValue};
	
// Inverted Section tags and End Section tags are used in combination to wrap a
// section of the template.
// 
// These tags' content MUST be a non-whitespace character sequence NOT
// containing the current closing delimiter; each Inverted Section tag MUST be
// followed by an End Section tag with the same content within the same
// section.
// 
// This tag's content names the data to replace the tag.  Name resolution is as
// follows:
//   1) Split the name on periods; the first part is the name to resolve, any
//   remaining parts should be retained.
//   2) Walk the context stack from top to bottom, finding the first context
//   that is a) a hash containing the name as a key OR b) an object responding
//   to a method with the given name.
//   3) If the context is a hash, the data is the value associated with the
//   name.
//   4) If the context is an object and the method with the given name has an
//   arity of 1, the method SHOULD be called with a String containing the
//   unprocessed contents of the sections; the data is the value returned.
//   5) Otherwise, the data is the value returned by calling the method with
//   the given name.
//   6) If any name parts were retained in step 1, each should be resolved
//   against a context stack containing only the result from the former
//   resolution.  If any part fails resolution, the result should be considered
//   falsey, and should interpolate as the empty string.
// If the data is not of a list type, it is coerced into a list as follows: if
// the data is truthy (e.g. `!!data == true`), use a single-element list
// containing the data, otherwise use an empty list.
// 
// This section MUST NOT be rendered unless the data list is empty.
// 
// Inverted Section and End Section tags SHOULD be treated as standalone when
// appropriate.

/// Falsey sections should have their contents rendered.
#[test]
pub fn falsey () {
	let template = "\"{{^boolean}}This should be rendered.{{/boolean}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"This should be rendered.\"");
	assert_eq!(result, expected)
}

/// Truthy sections should have their contents omitted.
#[test]
pub fn truthy () {
	let template = "\"{{^boolean}}This should not be rendered.{{/boolean}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"\"");
	assert_eq!(result, expected)
}

/// Null is falsey.
#[test]
pub fn null_is_falsey () {
	let template = "\"{{^null}}This should be rendered.{{/null}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("null",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"This should be rendered.\"");
	assert_eq!(result, expected)
}

/// Objects and hashes should behave like truthy values.
#[test]
pub fn context () {
	let template = "\"{{^context}}Hi {{name}}.{{/context}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"\"");
	assert_eq!(result, expected)
}

/// Lists should behave like truthy values.
#[test]
pub fn list () {
	let template = "\"{{^list}}{{n}}{{/list}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"\"");
	assert_eq!(result, expected)
}

/// Empty lists should behave like falsey values.
#[test]
pub fn empty_list () {
	let template = "\"{{^list}}Yay lists!{{/list}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"Yay lists!\"");
	assert_eq!(result, expected)
}

/// Multiple inverted sections per template should be permitted.
#[test]
pub fn doubled () {
	let template = "{{^bool}}\n* first\n{{/bool}}\n* {{two}}\n{{^bool}}\n* third\n{{/bool}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("bool",CtxValue::Boolean(false));
	ctx.insert("two",CtxValue::String("second".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("* first\n* second\n* third\n");
	assert_eq!(result, expected)
}

/// Nested falsey sections should have their contents rendered.
#[test]
pub fn nested_falsey () {
	let template = "| A {{^bool}}B {{^bool}}C{{/bool}} D{{/bool}} E |".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("bool",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("| A B C D E |");
	assert_eq!(result, expected)
}

/// Nested truthy sections should be omitted.
#[test]
pub fn nested_truthy () {
	let template = "| A {{^bool}}B {{^bool}}C{{/bool}} D{{/bool}} E |".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("bool",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("| A  E |");
	assert_eq!(result, expected)
}

/// Failed context lookups should be considered falsey.
#[test]
pub fn context_misses () {
	let template = "[{{^missing}}Cannot find key 'missing'!{{/missing}}]".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("[Cannot find key 'missing'!]");
	assert_eq!(result, expected)
}

/// Dotted names should be valid for Inverted Section tags.
#[test]
pub fn dotted_names__truthy () {
	let template = "\"{{^a.b.c}}Not Here{{/a.b.c}}\" == \"\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"\" == \"\"");
	assert_eq!(result, expected)
}

/// Dotted names should be valid for Inverted Section tags.
#[test]
pub fn dotted_names__falsey () {
	let template = "\"{{^a.b.c}}Not Here{{/a.b.c}}\" == \"Not Here\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"Not Here\" == \"Not Here\"");
	assert_eq!(result, expected)
}

/// Dotted names that cannot be resolved should be considered falsey.
#[test]
pub fn dotted_names__broken_chains () {
	let template = "\"{{^a.b.c}}Not Here{{/a.b.c}}\" == \"Not Here\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"Not Here\" == \"Not Here\"");
	assert_eq!(result, expected)
}

/// Inverted sections should not alter surrounding whitespace.
#[test]
pub fn surrounding_whitespace () {
	let template = " | {{^boolean}}\t|\t{{/boolean}} | \n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from(" | \t|\t | \n");
	assert_eq!(result, expected)
}

/// Inverted should not alter internal whitespace.
#[test]
pub fn internal_whitespace () {
	let template = " | {{^boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from(" |  \n  | \n");
	assert_eq!(result, expected)
}

/// Single-line sections should not alter surrounding whitespace.
#[test]
pub fn indented_inline_sections () {
	let template = " {{^boolean}}NO{{/boolean}}\n {{^boolean}}WAY{{/boolean}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from(" NO\n WAY\n");
	assert_eq!(result, expected)
}

/// Standalone lines should be removed from the template.
#[test]
pub fn standalone_lines () {
	let template = "| This Is\n{{^boolean}}\n|\n{{/boolean}}\n| A Line\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("| This Is\n|\n| A Line\n");
	assert_eq!(result, expected)
}

/// Standalone indented lines should be removed from the template.
#[test]
pub fn standalone_indented_lines () {
	let template = "| This Is\n  {{^boolean}}\n|\n  {{/boolean}}\n| A Line\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("| This Is\n|\n| A Line\n");
	assert_eq!(result, expected)
}

/// "\r\n" should be considered a newline for standalone tags.
#[test]
pub fn standalone_line_endings () {
	let template = "|\r\n{{^boolean}}\r\n{{/boolean}}\r\n|".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("|\r\n|");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to precede them.
#[test]
pub fn standalone_without_previous_line () {
	let template = "  {{^boolean}}\n^{{/boolean}}\n/".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("^\n/");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to follow them.
#[test]
pub fn standalone_without_newline () {
	let template = "^{{^boolean}}\n/\n  {{/boolean}}".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("^\n/\n");
	assert_eq!(result, expected)
}

/// Superfluous in-tag whitespace should be ignored.
#[test]
pub fn padding () {
	let template = "|{{^ boolean }}={{/ boolean }}|".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("|=|");
	assert_eq!(result, expected)
}
}