#[cfg(test)]
mod tests {
	use crate::html_templating::{TemplateEngine, OneoffRender, CtxValue};
	
// Section tags and End Section tags are used in combination to wrap a section
// of the template for iteration.
// 
// These tags' content MUST be a non-whitespace character sequence NOT
// containing the current closing delimiter; each Section tag MUST be followed
// by an End Section tag with the same content within the same section.
// 
// This tag's content names the data to replace the tag.  Name resolution is as
// follows:
//   1) If the name is a single period (.), the data is the item currently
//   sitting atop the context stack. Skip the rest of these steps.
//   2) Split the name on periods; the first part is the name to resolve, any
//   remaining parts should be retained.
//   3) Walk the context stack from top to bottom, finding the first context
//   that is a) a hash containing the name as a key OR b) an object responding
//   to a method with the given name.
//   4) If the context is a hash, the data is the value associated with the
//   name.
//   5) If the context is an object and the method with the given name has an
//   arity of 1, the method SHOULD be called with a String containing the
//   unprocessed contents of the sections; the data is the value returned.
//   6) Otherwise, the data is the value returned by calling the method with
//   the given name.
//   7) If any name parts were retained in step 1, each should be resolved
//   against a context stack containing only the result from the former
//   resolution.  If any part fails resolution, the result should be considered
//   falsey, and should interpolate as the empty string.
// 
// If the data is not of a list type, it is coerced into a list as follows: if
// the data is truthy (e.g. `!!data == true`), use a single-element list
// containing the data, otherwise use an empty list.
// 
// For each element in the data list, the element MUST be pushed onto the
// context stack, the section MUST be rendered, and the element MUST be popped
// off the context stack.
// 
// Section and End Section tags SHOULD be treated as standalone when
// appropriate.

/// Truthy sections should have their contents rendered.
#[test]
pub fn truthy () {
	let template = "\"{{#boolean}}This should be rendered.{{/boolean}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"This should be rendered.\"");
	assert_eq!(result, expected)
}

/// Falsey sections should have their contents omitted.
#[test]
pub fn falsey () {
	let template = "\"{{#boolean}}This should not be rendered.{{/boolean}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"\"");
	assert_eq!(result, expected)
}

/// Null is falsey.
#[test]
pub fn null_is_falsey () {
	let template = "\"{{#null}}This should not be rendered.{{/null}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("null",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"\"");
	assert_eq!(result, expected)
}

/// Objects and hashes should be pushed onto the context stack.
#[test]
pub fn context () {
	let template = "\"{{#context}}Hi {{name}}.{{/context}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"Hi Joe.\"");
	assert_eq!(result, expected)
}

/// Names missing in the current context are looked up in the stack.
#[test]
pub fn parent_contexts () {
	let template = "\"{{#sec}}{{a}}, {{b}}, {{c.d}}{{/sec}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("a",CtxValue::String("foo".to_string()));
	ctx.insert("b",CtxValue::String("wrong".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"foo, bar, baz\"");
	assert_eq!(result, expected)
}

/// Non-false sections have their value at the top of context,
/// accessible as {{.}} or through the parent context. This gives
/// a simple way to display content conditionally if a variable exists.
#[test]
pub fn variable_test () {
	let template = "\"{{#foo}}{{.}} is {{foo}}{{/foo}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("foo",CtxValue::String("bar".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"bar is bar\"");
	assert_eq!(result, expected)
}

/// All elements on the context stack should be accessible within lists.
#[test]
pub fn list_contexts () {
	let template = "{{#tops}}{{#middles}}{{tname.lower}}{{mname}}.{{#bottoms}}{{tname.upper}}{{mname}}{{bname}}.{{/bottoms}}{{/middles}}{{/tops}}".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("a1.A1x.A1y.");
	assert_eq!(result, expected)
}

/// All elements on the context stack should be accessible.
#[test]
pub fn deeply_nested_contexts () {
	let template = "{{#a}}\n{{one}}\n{{#b}}\n{{one}}{{two}}{{one}}\n{{#c}}\n{{one}}{{two}}{{three}}{{two}}{{one}}\n{{#d}}\n{{one}}{{two}}{{three}}{{four}}{{three}}{{two}}{{one}}\n{{#five}}\n{{one}}{{two}}{{three}}{{four}}{{five}}{{four}}{{three}}{{two}}{{one}}\n{{one}}{{two}}{{three}}{{four}}{{.}}6{{.}}{{four}}{{three}}{{two}}{{one}}\n{{one}}{{two}}{{three}}{{four}}{{five}}{{four}}{{three}}{{two}}{{one}}\n{{/five}}\n{{one}}{{two}}{{three}}{{four}}{{three}}{{two}}{{one}}\n{{/d}}\n{{one}}{{two}}{{three}}{{two}}{{one}}\n{{/c}}\n{{one}}{{two}}{{one}}\n{{/b}}\n{{one}}\n{{/a}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("1\n121\n12321\n1234321\n123454321\n12345654321\n123454321\n1234321\n12321\n121\n1\n");
	assert_eq!(result, expected)
}

/// Lists should be iterated; list items should visit the context stack.
#[test]
pub fn list () {
	let template = "\"{{#list}}{{item}}{{/list}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"123\"");
	assert_eq!(result, expected)
}

/// Empty lists should behave like falsey values.
#[test]
pub fn empty_list () {
	let template = "\"{{#list}}Yay lists!{{/list}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"\"");
	assert_eq!(result, expected)
}

/// Multiple sections per template should be permitted.
#[test]
pub fn doubled () {
	let template = "{{#bool}}\n* first\n{{/bool}}\n* {{two}}\n{{#bool}}\n* third\n{{/bool}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("bool",CtxValue::Boolean(true));
	ctx.insert("two",CtxValue::String("second".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("* first\n* second\n* third\n");
	assert_eq!(result, expected)
}

/// Nested truthy sections should have their contents rendered.
#[test]
pub fn nested_truthy () {
	let template = "| A {{#bool}}B {{#bool}}C{{/bool}} D{{/bool}} E |".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("bool",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("| A B C D E |");
	assert_eq!(result, expected)
}

/// Nested falsey sections should be omitted.
#[test]
pub fn nested_falsey () {
	let template = "| A {{#bool}}B {{#bool}}C{{/bool}} D{{/bool}} E |".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("bool",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("| A  E |");
	assert_eq!(result, expected)
}

/// Failed context lookups should be considered falsey.
#[test]
pub fn context_misses () {
	let template = "[{{#missing}}Found key 'missing'!{{/missing}}]".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("[]");
	assert_eq!(result, expected)
}

/// Implicit iterators should directly interpolate strings.
#[test]
pub fn implicit_iterator__string () {
	let template = "\"{{#list}}({{.}}){{/list}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"(a)(b)(c)(d)(e)\"");
	assert_eq!(result, expected)
}

/// Implicit iterators should cast integers to strings and interpolate.
#[test]
pub fn implicit_iterator__integer () {
	let template = "\"{{#list}}({{.}}){{/list}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"(1)(2)(3)(4)(5)\"");
	assert_eq!(result, expected)
}

/// Implicit iterators should cast decimals to strings and interpolate.
#[test]
pub fn implicit_iterator__decimal () {
	let template = "\"{{#list}}({{.}}){{/list}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"(1.1)(2.2)(3.3)(4.4)(5.5)\"");
	assert_eq!(result, expected)
}

/// Implicit iterators should allow iterating over nested arrays.
#[test]
pub fn implicit_iterator__array () {
	let template = "\"{{#list}}({{#.}}{{.}}{{/.}}){{/list}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"(123)(abc)\"");
	assert_eq!(result, expected)
}

/// Implicit iterators with basic interpolation should be HTML escaped.
#[test]
pub fn implicit_iterator__html_escaping () {
	let template = "\"{{#list}}({{.}}){{/list}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"(&amp;)(&quot;)(&lt;)(&gt;)\"");
	assert_eq!(result, expected)
}

/// Implicit iterators in triple mustache should interpolate without HTML escaping.
#[test]
pub fn implicit_iterator__triple_mustache () {
	let template = "\"{{#list}}({{{.}}}){{/list}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"(&)(\")(<)(>)\"");
	assert_eq!(result, expected)
}

/// Implicit iterators in an Ampersand tag should interpolate without HTML escaping.
#[test]
pub fn implicit_iterator__ampersand () {
	let template = "\"{{#list}}({{&.}}){{/list}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"(&)(\")(<)(>)\"");
	assert_eq!(result, expected)
}

/// Implicit iterators should work on root-level lists.
#[test]
pub fn implicit_iterator__rootlevel () {
	let template = "\"{{#.}}({{value}}){{/.}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert(".0.value",CtxValue::String("a".to_string()));
	ctx.insert(".1.value",CtxValue::String("b".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"(a)(b)\"");
	assert_eq!(result, expected)
}

/// Dotted names should be valid for Section tags.
#[test]
pub fn dotted_names__truthy () {
	let template = "\"{{#a.b.c}}Here{{/a.b.c}}\" == \"Here\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"Here\" == \"Here\"");
	assert_eq!(result, expected)
}

/// Dotted names should be valid for Section tags.
#[test]
pub fn dotted_names__falsey () {
	let template = "\"{{#a.b.c}}Here{{/a.b.c}}\" == \"\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"\" == \"\"");
	assert_eq!(result, expected)
}

/// Dotted names that cannot be resolved should be considered falsey.
#[test]
pub fn dotted_names__broken_chains () {
	let template = "\"{{#a.b.c}}Here{{/a.b.c}}\" == \"\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"\" == \"\"");
	assert_eq!(result, expected)
}

/// Sections should not alter surrounding whitespace.
#[test]
pub fn surrounding_whitespace () {
	let template = " | {{#boolean}}\t|\t{{/boolean}} | \n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from(" | \t|\t | \n");
	assert_eq!(result, expected)
}

/// Sections should not alter internal whitespace.
#[test]
pub fn internal_whitespace () {
	let template = " | {{#boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from(" |  \n  | \n");
	assert_eq!(result, expected)
}

/// Single-line sections should not alter surrounding whitespace.
#[test]
pub fn indented_inline_sections () {
	let template = " {{#boolean}}YES{{/boolean}}\n {{#boolean}}GOOD{{/boolean}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from(" YES\n GOOD\n");
	assert_eq!(result, expected)
}

/// Standalone lines should be removed from the template.
#[test]
pub fn standalone_lines () {
	let template = "| This Is\n{{#boolean}}\n|\n{{/boolean}}\n| A Line\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("| This Is\n|\n| A Line\n");
	assert_eq!(result, expected)
}

/// Indented standalone lines should be removed from the template.
#[test]
pub fn indented_standalone_lines () {
	let template = "| This Is\n  {{#boolean}}\n|\n  {{/boolean}}\n| A Line\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("| This Is\n|\n| A Line\n");
	assert_eq!(result, expected)
}

/// "\r\n" should be considered a newline for standalone tags.
#[test]
pub fn standalone_line_endings () {
	let template = "|\r\n{{#boolean}}\r\n{{/boolean}}\r\n|".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("|\r\n|");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to precede them.
#[test]
pub fn standalone_without_previous_line () {
	let template = "  {{#boolean}}\n#{{/boolean}}\n/".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("#\n/");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to follow them.
#[test]
pub fn standalone_without_newline () {
	let template = "#{{#boolean}}\n/\n  {{/boolean}}".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("#\n/\n");
	assert_eq!(result, expected)
}

/// Superfluous in-tag whitespace should be ignored.
#[test]
pub fn padding () {
	let template = "|{{# boolean }}={{/ boolean }}|".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("boolean",CtxValue::Boolean(true));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("|=|");
	assert_eq!(result, expected)
}
}