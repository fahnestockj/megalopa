#[cfg(test)]
mod tests {
	use crate::html_templating::{create_oneoff_engine, oneoff_render};
	

/// The greater-than operator should expand to the named partial.
#[test]
pub fn basic_behavior () {
	let template = "\"{{>text}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\"from partial\"");
	assert_eq!(result, expected)
}

/// The empty string should be used when the named partial is not found.
#[test]
pub fn failed_lookup () {
	let template = "\"{{>text}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\"\"");
	assert_eq!(result, expected)
}

/// The greater-than operator should operate within the current context.
#[test]
pub fn context () {
	let template = "\"{{>partial}}\"";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".text","content");
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\"*content*\"");
	assert_eq!(result, expected)
}

/// The greater-than operator should properly recurse.
#[test]
pub fn recursion () {
	let template = "{{>node}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".content","X");
	let result = engine.oneoff_render(ctx);
	let expected = String::from("X<Y<>>");
	assert_eq!(result, expected)
}

/// The greater-than operator should work from within partials.
#[test]
pub fn nested () {
	let template = "{{>outer}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".a","hello");
	ctx.insert(".b","world");
	let result = engine.oneoff_render(ctx);
	let expected = String::from("*hello world!*");
	assert_eq!(result, expected)
}

/// The greater-than operator should not alter surrounding whitespace.
#[test]
pub fn surrounding_whitespace () {
	let template = "| {{>partial}} |";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("| \t|\t |");
	assert_eq!(result, expected)
}

/// Whitespace should be left untouched.
#[test]
pub fn inline_indentation () {
	let template = "  {{data}}  {{> partial}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".data","|");
	let result = engine.oneoff_render(ctx);
	let expected = String::from("  |  >\n>\n");
	assert_eq!(result, expected)
}

/// "\r\n" should be considered a newline for standalone tags.
#[test]
pub fn standalone_line_endings () {
	let template = "|\r\n{{>partial}}\r\n|";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("|\r\n>|");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to precede them.
#[test]
pub fn standalone_without_previous_line () {
	let template = "  {{>partial}}\n>";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("  >\n  >>");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to follow them.
#[test]
pub fn standalone_without_newline () {
	let template = ">\n  {{>partial}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from(">\n  >\n  >");
	assert_eq!(result, expected)
}

/// Each line of the partial should be indented before rendering.
#[test]
pub fn standalone_indentation () {
	let template = "\\\n {{>partial}}\n/\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".content","<\n->");
	let result = engine.oneoff_render(ctx);
	let expected = String::from("\\\n |\n <\n->\n |\n/\n");
	assert_eq!(result, expected)
}

/// Superfluous in-tag whitespace should be ignored.
#[test]
pub fn padding_whitespace () {
	let template = "|{{> partial }}|";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".boolean",true);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("|[]|");
	assert_eq!(result, expected)
}
}