#[cfg(test)]
mod tests {
	use crate::html_templating::{create_oneoff_engine, oneoff_render};
	

/// Falsey sections should have their contents rendered.
#[test]
pub fn falsey () {
	let template = "\"{{^boolean}}This should be rendered.{{/boolean}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".boolean",false);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\"This should be rendered.\"");
	assert_eq!(result, expected)
}

/// Truthy sections should have their contents omitted.
#[test]
pub fn truthy () {
	let template = "\"{{^boolean}}This should not be rendered.{{/boolean}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".boolean",true);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\"\"");
	assert_eq!(result, expected)
}

/// Null is falsey.
#[test]
pub fn null_is_falsey () {
	let template = "\"{{^null}}This should be rendered.{{/null}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".null",null);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\"This should be rendered.\"");
	assert_eq!(result, expected)
}

/// Objects and hashes should behave like truthy values.
#[test]
pub fn context () {
	let template = "\"{{^context}}Hi {{name}}.{{/context}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\"\"");
	assert_eq!(result, expected)
}

/// Lists should behave like truthy values.
#[test]
pub fn list () {
	let template = "\"{{^list}}{{n}}{{/list}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\"\"");
	assert_eq!(result, expected)
}

/// Empty lists should behave like falsey values.
#[test]
pub fn empty_list () {
	let template = "\"{{^list}}Yay lists!{{/list}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\"Yay lists!\"");
	assert_eq!(result, expected)
}

/// Multiple inverted sections per template should be permitted.
#[test]
pub fn doubled () {
	let template = "{{^bool}}\n* first\n{{/bool}}\n* {{two}}\n{{^bool}}\n* third\n{{/bool}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".bool",false);
	ctx.insert(".two","second");
	let result = engine.oneoff_render(ctx);
	let expected = String::from("* first\n* second\n* third\n");
	assert_eq!(result, expected)
}

/// Nested falsey sections should have their contents rendered.
#[test]
pub fn nested_falsey () {
	let template = "| A {{^bool}}B {{^bool}}C{{/bool}} D{{/bool}} E |";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".bool",false);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("| A B C D E |");
	assert_eq!(result, expected)
}

/// Nested truthy sections should be omitted.
#[test]
pub fn nested_truthy () {
	let template = "| A {{^bool}}B {{^bool}}C{{/bool}} D{{/bool}} E |";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".bool",true);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("| A  E |");
	assert_eq!(result, expected)
}

/// Failed context lookups should be considered falsey.
#[test]
pub fn context_misses () {
	let template = "[{{^missing}}Cannot find key 'missing'!{{/missing}}]";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("[Cannot find key 'missing'!]");
	assert_eq!(result, expected)
}

/// Dotted names should be valid for Inverted Section tags.
#[test]
pub fn dotted_names__truthy () {
	let template = "\"{{^a.b.c}}Not Here{{/a.b.c}}\" == \"\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\"\" == \"\"");
	assert_eq!(result, expected)
}

/// Dotted names should be valid for Inverted Section tags.
#[test]
pub fn dotted_names__falsey () {
	let template = "\"{{^a.b.c}}Not Here{{/a.b.c}}\" == \"Not Here\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\"Not Here\" == \"Not Here\"");
	assert_eq!(result, expected)
}

/// Dotted names that cannot be resolved should be considered falsey.
#[test]
pub fn dotted_names__broken_chains () {
	let template = "\"{{^a.b.c}}Not Here{{/a.b.c}}\" == \"Not Here\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\"Not Here\" == \"Not Here\"");
	assert_eq!(result, expected)
}

/// Inverted sections should not alter surrounding whitespace.
#[test]
pub fn surrounding_whitespace () {
	let template = " | {{^boolean}}\t|\t{{/boolean}} | \n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".boolean",false);
	let result = engine.oneoff_render(ctx);
	let expected = String::from(" | \t|\t | \n");
	assert_eq!(result, expected)
}

/// Inverted should not alter internal whitespace.
#[test]
pub fn internal_whitespace () {
	let template = " | {{^boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".boolean",false);
	let result = engine.oneoff_render(ctx);
	let expected = String::from(" |  \n  | \n");
	assert_eq!(result, expected)
}

/// Single-line sections should not alter surrounding whitespace.
#[test]
pub fn indented_inline_sections () {
	let template = " {{^boolean}}NO{{/boolean}}\n {{^boolean}}WAY{{/boolean}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".boolean",false);
	let result = engine.oneoff_render(ctx);
	let expected = String::from(" NO\n WAY\n");
	assert_eq!(result, expected)
}

/// Standalone lines should be removed from the template.
#[test]
pub fn standalone_lines () {
	let template = "| This Is\n{{^boolean}}\n|\n{{/boolean}}\n| A Line\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".boolean",false);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("| This Is\n|\n| A Line\n");
	assert_eq!(result, expected)
}

/// Standalone indented lines should be removed from the template.
#[test]
pub fn standalone_indented_lines () {
	let template = "| This Is\n  {{^boolean}}\n|\n  {{/boolean}}\n| A Line\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".boolean",false);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("| This Is\n|\n| A Line\n");
	assert_eq!(result, expected)
}

/// "\r\n" should be considered a newline for standalone tags.
#[test]
pub fn standalone_line_endings () {
	let template = "|\r\n{{^boolean}}\r\n{{/boolean}}\r\n|";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".boolean",false);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("|\r\n|");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to precede them.
#[test]
pub fn standalone_without_previous_line () {
	let template = "  {{^boolean}}\n^{{/boolean}}\n/";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".boolean",false);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("^\n/");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to follow them.
#[test]
pub fn standalone_without_newline () {
	let template = "^{{^boolean}}\n/\n  {{/boolean}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".boolean",false);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("^\n/\n");
	assert_eq!(result, expected)
}

/// Superfluous in-tag whitespace should be ignored.
#[test]
pub fn padding () {
	let template = "|{{^ boolean }}={{/ boolean }}|";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".boolean",false);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("|=|");
	assert_eq!(result, expected)
}
}