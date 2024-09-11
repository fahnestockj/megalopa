#[cfg(test)]
mod tests {
	use crate::html_templating::{TemplateEngine, OneoffRender, CtxValue};
	

/// Mustache-free templates should render as-is.
#[test]
pub fn no_interpolation () {
	let template = "Hello from {Mustache}!\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("Hello from {Mustache}!\n");
	assert_eq!(result, expected)
}

/// Unadorned tags should interpolate content into the template.
#[test]
pub fn basic_interpolation () {
	let template = "Hello, {{subject}}!\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("subject",CtxValue::String("world".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("Hello, world!\n");
	assert_eq!(result, expected)
}

/// Interpolated tag output should not be re-interpolated.
#[test]
pub fn no_reinterpolation () {
	let template = "{{template}}: {{planet}}".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("template",CtxValue::String("{{planet}}".to_string()));
	ctx.insert("planet",CtxValue::String("Earth".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("{{planet}}: Earth");
	assert_eq!(result, expected)
}

/// Basic interpolation should be HTML escaped.
#[test]
pub fn html_escaping () {
	let template = "These characters should be HTML escaped: {{forbidden}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("forbidden",CtxValue::String("& \" < >".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("These characters should be HTML escaped: &amp; &quot; &lt; &gt;\n");
	assert_eq!(result, expected)
}

/// Triple mustaches should interpolate without HTML escaping.
#[test]
pub fn triple_mustache () {
	let template = "These characters should not be HTML escaped: {{{forbidden}}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("forbidden",CtxValue::String("& \" < >".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("These characters should not be HTML escaped: & \" < >\n");
	assert_eq!(result, expected)
}

/// Ampersand should interpolate without HTML escaping.
#[test]
pub fn ampersand () {
	let template = "These characters should not be HTML escaped: {{&forbidden}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("forbidden",CtxValue::String("& \" < >".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("These characters should not be HTML escaped: & \" < >\n");
	assert_eq!(result, expected)
}

/// Integers should interpolate seamlessly.
#[test]
pub fn basic_integer_interpolation () {
	let template = "\"{{mph}} miles an hour!\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("mph",CtxValue::Number(85));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"85 miles an hour!\"");
	assert_eq!(result, expected)
}

/// Integers should interpolate seamlessly.
#[test]
pub fn triple_mustache_integer_interpolation () {
	let template = "\"{{{mph}}} miles an hour!\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("mph",CtxValue::Number(85));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"85 miles an hour!\"");
	assert_eq!(result, expected)
}

/// Integers should interpolate seamlessly.
#[test]
pub fn ampersand_integer_interpolation () {
	let template = "\"{{&mph}} miles an hour!\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("mph",CtxValue::Number(85));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"85 miles an hour!\"");
	assert_eq!(result, expected)
}

/// Decimals should interpolate seamlessly with proper significance.
#[test]
pub fn basic_decimal_interpolation () {
	let template = "\"{{power}} jiggawatts!\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("power",CtxValue::String(1.21.to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"1.21 jiggawatts!\"");
	assert_eq!(result, expected)
}

/// Decimals should interpolate seamlessly with proper significance.
#[test]
pub fn triple_mustache_decimal_interpolation () {
	let template = "\"{{{power}}} jiggawatts!\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("power",CtxValue::String(1.21.to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"1.21 jiggawatts!\"");
	assert_eq!(result, expected)
}

/// Decimals should interpolate seamlessly with proper significance.
#[test]
pub fn ampersand_decimal_interpolation () {
	let template = "\"{{&power}} jiggawatts!\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("power",CtxValue::String(1.21.to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"1.21 jiggawatts!\"");
	assert_eq!(result, expected)
}

/// Nulls should interpolate as the empty string.
#[test]
pub fn basic_null_interpolation () {
	let template = "I ({{cannot}}) be seen!".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("cannot",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("I () be seen!");
	assert_eq!(result, expected)
}

/// Nulls should interpolate as the empty string.
#[test]
pub fn triple_mustache_null_interpolation () {
	let template = "I ({{{cannot}}}) be seen!".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("cannot",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("I () be seen!");
	assert_eq!(result, expected)
}

/// Nulls should interpolate as the empty string.
#[test]
pub fn ampersand_null_interpolation () {
	let template = "I ({{&cannot}}) be seen!".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("cannot",CtxValue::Boolean(false));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("I () be seen!");
	assert_eq!(result, expected)
}

/// Failed context lookups should default to empty strings.
#[test]
pub fn basic_context_miss_interpolation () {
	let template = "I ({{cannot}}) be seen!".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("I () be seen!");
	assert_eq!(result, expected)
}

/// Failed context lookups should default to empty strings.
#[test]
pub fn triple_mustache_context_miss_interpolation () {
	let template = "I ({{{cannot}}}) be seen!".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("I () be seen!");
	assert_eq!(result, expected)
}

/// Failed context lookups should default to empty strings.
#[test]
pub fn ampersand_context_miss_interpolation () {
	let template = "I ({{&cannot}}) be seen!".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("I () be seen!");
	assert_eq!(result, expected)
}

/// Dotted names should be considered a form of shorthand for sections.
#[test]
pub fn dotted_names__basic_interpolation () {
	let template = "\"{{person.name}}\" == \"{{#person}}{{name}}{{/person}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"Joe\" == \"Joe\"");
	assert_eq!(result, expected)
}

/// Dotted names should be considered a form of shorthand for sections.
#[test]
pub fn dotted_names__triple_mustache_interpolation () {
	let template = "\"{{{person.name}}}\" == \"{{#person}}{{{name}}}{{/person}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"Joe\" == \"Joe\"");
	assert_eq!(result, expected)
}

/// Dotted names should be considered a form of shorthand for sections.
#[test]
pub fn dotted_names__ampersand_interpolation () {
	let template = "\"{{&person.name}}\" == \"{{#person}}{{&name}}{{/person}}\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"Joe\" == \"Joe\"");
	assert_eq!(result, expected)
}

/// Dotted names should be functional to any level of nesting.
#[test]
pub fn dotted_names__arbitrary_depth () {
	let template = "\"{{a.b.c.d.e.name}}\" == \"Phil\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"Phil\" == \"Phil\"");
	assert_eq!(result, expected)
}

/// Any falsey value prior to the last part of the name should yield ''.
#[test]
pub fn dotted_names__broken_chains () {
	let template = "\"{{a.b.c}}\" == \"\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"\" == \"\"");
	assert_eq!(result, expected)
}

/// Each part of a dotted name should resolve only against its parent.
#[test]
pub fn dotted_names__broken_chain_resolution () {
	let template = "\"{{a.b.c.name}}\" == \"\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"\" == \"\"");
	assert_eq!(result, expected)
}

/// The first part of a dotted name should resolve as any other name.
#[test]
pub fn dotted_names__initial_resolution () {
	let template = "\"{{#a}}{{b.c.d.e.name}}{{/a}}\" == \"Phil\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"Phil\" == \"Phil\"");
	assert_eq!(result, expected)
}

/// Dotted names should be resolved against former resolutions.
#[test]
pub fn dotted_names__context_precedence () {
	let template = "{{#a}}{{b.c}}{{/a}}".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("");
	assert_eq!(result, expected)
}

/// Dotted names shall not be parsed as single, atomic keys
#[test]
pub fn dotted_names_are_never_single_keys () {
	let template = "{{a.b}}".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("a.b",CtxValue::String("c".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("");
	assert_eq!(result, expected)
}

/// Dotted Names in a given context are unvavailable due to dot splitting
#[test]
pub fn dotted_names__no_masking () {
	let template = "{{a.b}}".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("a.b",CtxValue::String("c".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("d");
	assert_eq!(result, expected)
}

/// Unadorned tags should interpolate content into the template.
#[test]
pub fn implicit_iterators__basic_interpolation () {
	let template = "Hello, {{.}}!\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert(".",CtxValue::String("world".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("Hello, world!\n");
	assert_eq!(result, expected)
}

/// Basic interpolation should be HTML escaped.
#[test]
pub fn implicit_iterators__html_escaping () {
	let template = "These characters should be HTML escaped: {{.}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert(".",CtxValue::String("& \" < >".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("These characters should be HTML escaped: &amp; &quot; &lt; &gt;\n");
	assert_eq!(result, expected)
}

/// Triple mustaches should interpolate without HTML escaping.
#[test]
pub fn implicit_iterators__triple_mustache () {
	let template = "These characters should not be HTML escaped: {{{.}}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert(".",CtxValue::String("& \" < >".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("These characters should not be HTML escaped: & \" < >\n");
	assert_eq!(result, expected)
}

/// Ampersand should interpolate without HTML escaping.
#[test]
pub fn implicit_iterators__ampersand () {
	let template = "These characters should not be HTML escaped: {{&.}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert(".",CtxValue::String("& \" < >".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("These characters should not be HTML escaped: & \" < >\n");
	assert_eq!(result, expected)
}

/// Integers should interpolate seamlessly.
#[test]
pub fn implicit_iterators__basic_integer_interpolation () {
	let template = "\"{{.}} miles an hour!\"".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert(".",CtxValue::Number(85));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("\"85 miles an hour!\"");
	assert_eq!(result, expected)
}

/// Interpolation should not alter surrounding whitespace.
#[test]
pub fn interpolation__surrounding_whitespace () {
	let template = "| {{string}} |".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("string",CtxValue::String("---".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("| --- |");
	assert_eq!(result, expected)
}

/// Interpolation should not alter surrounding whitespace.
#[test]
pub fn triple_mustache__surrounding_whitespace () {
	let template = "| {{{string}}} |".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("string",CtxValue::String("---".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("| --- |");
	assert_eq!(result, expected)
}

/// Interpolation should not alter surrounding whitespace.
#[test]
pub fn ampersand__surrounding_whitespace () {
	let template = "| {{&string}} |".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("string",CtxValue::String("---".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("| --- |");
	assert_eq!(result, expected)
}

/// Standalone interpolation should not alter surrounding whitespace.
#[test]
pub fn interpolation__standalone () {
	let template = "  {{string}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("string",CtxValue::String("---".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("  ---\n");
	assert_eq!(result, expected)
}

/// Standalone interpolation should not alter surrounding whitespace.
#[test]
pub fn triple_mustache__standalone () {
	let template = "  {{{string}}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("string",CtxValue::String("---".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("  ---\n");
	assert_eq!(result, expected)
}

/// Standalone interpolation should not alter surrounding whitespace.
#[test]
pub fn ampersand__standalone () {
	let template = "  {{&string}}\n".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("string",CtxValue::String("---".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("  ---\n");
	assert_eq!(result, expected)
}

/// Superfluous in-tag whitespace should be ignored.
#[test]
pub fn interpolation_with_padding () {
	let template = "|{{ string }}|".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("string",CtxValue::String("---".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("|---|");
	assert_eq!(result, expected)
}

/// Superfluous in-tag whitespace should be ignored.
#[test]
pub fn triple_mustache_with_padding () {
	let template = "|{{{ string }}}|".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("string",CtxValue::String("---".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("|---|");
	assert_eq!(result, expected)
}

/// Superfluous in-tag whitespace should be ignored.
#[test]
pub fn ampersand_with_padding () {
	let template = "|{{& string }}|".to_string();
	let engine = TemplateEngine{};
	let mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();
	ctx.insert("string",CtxValue::String("---".to_string()));
	let result = engine.oneoff_render(template, ctx);
	let expected = String::from("|---|");
	assert_eq!(result, expected)
}
}