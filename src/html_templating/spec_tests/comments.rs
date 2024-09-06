#[cfg(test)]
mod tests {
	use crate::html_templating::{create_oneoff_engine, oneoff_render};
	

/// Comment blocks should be removed from the template.
#[test]
pub fn inline () {
	let template = "12345{{! Comment Block! }}67890";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("1234567890");
	assert_eq!(result, expected)
}

/// Multiline comments should be permitted.
#[test]
pub fn multiline () {
	let template = "12345{{!\n  This is a\n  multi-line comment...\n}}67890\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("1234567890\n");
	assert_eq!(result, expected)
}

/// All standalone comment lines should be removed.
#[test]
pub fn standalone () {
	let template = "Begin.\n{{! Comment Block! }}\nEnd.\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("Begin.\nEnd.\n");
	assert_eq!(result, expected)
}

/// All standalone comment lines should be removed.
#[test]
pub fn indented_standalone () {
	let template = "Begin.\n  {{! Indented Comment Block! }}\nEnd.\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("Begin.\nEnd.\n");
	assert_eq!(result, expected)
}

/// "\r\n" should be considered a newline for standalone tags.
#[test]
pub fn standalone_line_endings () {
	let template = "|\r\n{{! Standalone Comment }}\r\n|";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("|\r\n|");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to precede them.
#[test]
pub fn standalone_without_previous_line () {
	let template = "  {{! I'm Still Standalone }}\n!";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("!");
	assert_eq!(result, expected)
}

/// Standalone tags should not require a newline to follow them.
#[test]
pub fn standalone_without_newline () {
	let template = "!\n  {{! I'm Still Standalone }}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("!\n");
	assert_eq!(result, expected)
}

/// All standalone comment lines should be removed.
#[test]
pub fn multiline_standalone () {
	let template = "Begin.\n{{!\nSomething's going on here...\n}}\nEnd.\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("Begin.\nEnd.\n");
	assert_eq!(result, expected)
}

/// All standalone comment lines should be removed.
#[test]
pub fn indented_multiline_standalone () {
	let template = "Begin.\n  {{!\n    Something's going on here...\n  }}\nEnd.\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("Begin.\nEnd.\n");
	assert_eq!(result, expected)
}

/// Inline comments should not strip whitespace
#[test]
pub fn indented_inline () {
	let template = "  12 {{! 34 }}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("  12 \n");
	assert_eq!(result, expected)
}

/// Comment removal should preserve surrounding whitespace.
#[test]
pub fn surrounding_whitespace () {
	let template = "12345 {{! Comment Block! }} 67890";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.oneoff_render(ctx);
	let expected = String::from("12345  67890");
	assert_eq!(result, expected)
}

/// Comments must never render, even if variable with same name exists.
#[test]
pub fn variable_name_collision () {
	let template = "comments never show: >{{! comment }}<";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".! comment",1);
	ctx.insert(".! comment ",2);
	ctx.insert(".!comment",3);
	ctx.insert(".comment",4);
	let result = engine.oneoff_render(ctx);
	let expected = String::from("comments never show: ><");
	assert_eq!(result, expected)
}
}