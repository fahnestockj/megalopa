#[cfg(test)]
mod tests {
	

/// The asterisk operator is used for dynamic partials.
#[test]
pub fn basic_behavior__partial () {
	let template = "\"{{>*dynamic}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".dynamic","content");
	let result = engine.render(ctx);
	let expected = String::from("\"Hello, world!\"");
	assert_eq(result, expected)
}

/// The asterisk is not part of the name that will be resolved in the context.
#[test]
pub fn basic_behavior__name_resolution () {
	let template = "\"{{>*dynamic}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".dynamic","content");
	ctx.insert(".*dynamic","wrong");
	let result = engine.render(ctx);
	let expected = String::from("\"Hello, world!\"");
	assert_eq(result, expected)
}

/// Failed context lookups should be considered falsey.
#[test]
pub fn context_misses__partial () {
	let template = "\"{{>*missing}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("\"\"");
	assert_eq(result, expected)
}

/// The empty string should be used when the named partial is not found.
#[test]
pub fn failed_lookup__partial () {
	let template = "\"{{>*dynamic}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".dynamic","content");
	let result = engine.render(ctx);
	let expected = String::from("\"\"");
	assert_eq(result, expected)
}

/// The dynamic partial should operate within the current context.
#[test]
pub fn context () {
	let template = "\"{{>*example}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".text","Hello, world!");
	ctx.insert(".example","partial");
	let result = engine.render(ctx);
	let expected = String::from("\"*Hello, world!*\"");
	assert_eq(result, expected)
}

/// The dynamic partial should operate within the current context.
#[test]
pub fn dotted_names () {
	let template = "\"{{>*foo.bar.baz}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".text","Hello, world!");
	let result = engine.render(ctx);
	let expected = String::from("\"*Hello, world!*\"");
	assert_eq(result, expected)
}

/// The dotted name should be resolved entirely before being dereferenced.
#[test]
pub fn dotted_names__operator_precedence () {
	let template = "\"{{>*foo.bar.baz}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".text","Hello, world!");
	ctx.insert(".foo","test");
	let result = engine.render(ctx);
	let expected = String::from("\"\"");
	assert_eq(result, expected)
}

/// The dynamic partial should operate within the current context.
#[test]
pub fn dotted_names__failed_lookup () {
	let template = "\"{{>*foo.bar.baz}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("\"**\"");
	assert_eq(result, expected)
}

/// Dotted names should not push a new frame on the context stack.
#[test]
pub fn dotted_names__context_stacking () {
	let template = "{{#section1}}{{>*section2.dynamic}}{{/section1}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("\"section1\"");
	assert_eq(result, expected)
}

/// Dotted names should not push a new frame on the context stack.
#[test]
pub fn dotted_names__context_stacking_under_repetition () {
	let template = "{{#section1}}{{>*section2.dynamic}}{{/section1}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".value","test");
	let result = engine.render(ctx);
	let expected = String::from("testtest");
	assert_eq(result, expected)
}

/// Dotted names should resolve against the proper context stack.
#[test]
pub fn dotted_names__context_stacking_failed_lookup () {
	let template = "{{#section1}}{{>*section2.dynamic}}{{/section1}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("\"\"\"\"");
	assert_eq(result, expected)
}

/// Dynamic partials should properly recurse.
#[test]
pub fn recursion () {
	let template = "{{>*template}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".template","node");
	ctx.insert(".content","X");
	let result = engine.render(ctx);
	let expected = String::from("X<Y<>>");
	assert_eq(result, expected)
}

/// Dynamic Names can't be dereferenced more than once.
#[test]
pub fn dynamic_names__double_dereferencing () {
	let template = "\"{{>**dynamic}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".dynamic","test");
	ctx.insert(".test","content");
	let result = engine.render(ctx);
	let expected = String::from("\"\"");
	assert_eq(result, expected)
}

/// Dotted Names are resolved entirely before dereferencing begins.
#[test]
pub fn dynamic_names__composed_dereferencing () {
	let template = "\"{{>*foo.*bar}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".foo","fizz");
	ctx.insert(".bar","buzz");
	let result = engine.render(ctx);
	let expected = String::from("\"\"");
	assert_eq(result, expected)
}

/// A dynamic partial should not alter surrounding whitespace; any
/// whitespace preceding the tag should be treated as indentation while any
/// whitespace succeding the tag should be left untouched.
#[test]
pub fn surrounding_whitespace () {
	let template = "| {{>*partial}} |";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".partial","foobar");
	let result = engine.render(ctx);
	let expected = String::from("| \t|\t |");
	assert_eq(result, expected)
}

/// Whitespace should be left untouched: whitespaces preceding the tag
/// should be treated as indentation.
#[test]
pub fn inline_indentation () {
	let template = "  {{data}}  {{>*dynamic}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".dynamic","partial");
	ctx.insert(".data","|");
	let result = engine.render(ctx);
	let expected = String::from("  |  >\n>\n");
	assert_eq(result, expected)
}

/// "\r\n" should be considered a newline for standalone tags.
#[test]
pub fn standalone_line_endings () {
	let template = "|\r\n{{>*dynamic}}\r\n|";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".dynamic","partial");
	let result = engine.render(ctx);
	let expected = String::from("|\r\n>|");
	assert_eq(result, expected)
}

/// Standalone tags should not require a newline to precede them.
#[test]
pub fn standalone_without_previous_line () {
	let template = "  {{>*dynamic}}\n>";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".dynamic","partial");
	let result = engine.render(ctx);
	let expected = String::from("  >\n  >>");
	assert_eq(result, expected)
}

/// Standalone tags should not require a newline to follow them.
#[test]
pub fn standalone_without_newline () {
	let template = ">\n  {{>*dynamic}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".dynamic","partial");
	let result = engine.render(ctx);
	let expected = String::from(">\n  >\n  >");
	assert_eq(result, expected)
}

/// Each line of the partial should be indented before rendering.
#[test]
pub fn standalone_indentation () {
	let template = "\\\n {{>*dynamic}}\n/\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".dynamic","partial");
	ctx.insert(".content","<\n->");
	let result = engine.render(ctx);
	let expected = String::from("\\\n |\n <\n->\n |\n/\n");
	assert_eq(result, expected)
}

/// Superfluous in-tag whitespace should be ignored.
#[test]
pub fn padding_whitespace () {
	let template = "|{{> * dynamic }}|";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".dynamic","partial");
	ctx.insert(".boolean",true);
	let result = engine.render(ctx);
	let expected = String::from("|[]|");
	assert_eq(result, expected)
}
}