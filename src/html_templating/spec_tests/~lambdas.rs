#[cfg(test)]
mod tests {
	

/// A lambda's return value should be interpolated.
#[test]
pub fn interpolation () {
	let template = "Hello, {{lambda}}!";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("Hello, world!");
	assert_eq(result, expected)
}

/// A lambda's return value should be parsed.
#[test]
pub fn interpolation__expansion () {
	let template = "Hello, {{lambda}}!";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".planet","world");
	let result = engine.render(ctx);
	let expected = String::from("Hello, world!");
	assert_eq(result, expected)
}

/// A lambda's return value should parse with the default delimiters.
#[test]
pub fn interpolation__alternate_delimiters () {
	let template = "{{= | | =}}\nHello, (|&lambda|)!";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".planet","world");
	let result = engine.render(ctx);
	let expected = String::from("Hello, (|planet| => world)!");
	assert_eq(result, expected)
}

/// Interpolated lambdas should not be cached.
#[test]
pub fn interpolation__multiple_calls () {
	let template = "{{lambda}} == {{{lambda}}} == {{lambda}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("1 == 2 == 3");
	assert_eq(result, expected)
}

/// Lambda results should be appropriately escaped.
#[test]
pub fn escaping () {
	let template = "<{{lambda}}{{{lambda}}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("<&gt;>");
	assert_eq(result, expected)
}

/// Lambdas used for sections should receive the raw section string.
#[test]
pub fn section () {
	let template = "<{{#lambda}}{{x}}{{/lambda}}>";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".x","Error!");
	let result = engine.render(ctx);
	let expected = String::from("<yes>");
	assert_eq(result, expected)
}

/// Lambdas used for sections should have their results parsed.
#[test]
pub fn section__expansion () {
	let template = "<{{#lambda}}-{{/lambda}}>";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".planet","Earth");
	let result = engine.render(ctx);
	let expected = String::from("<-Earth->");
	assert_eq(result, expected)
}

/// Lambdas used for sections should parse with the current delimiters.
#[test]
pub fn section__alternate_delimiters () {
	let template = "{{= | | =}}<|#lambda|-|/lambda|>";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".planet","Earth");
	let result = engine.render(ctx);
	let expected = String::from("<-{{planet}} => Earth->");
	assert_eq(result, expected)
}

/// Lambdas used for sections should not be cached.
#[test]
pub fn section__multiple_calls () {
	let template = "{{#lambda}}FILE{{/lambda}} != {{#lambda}}LINE{{/lambda}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("__FILE__ != __LINE__");
	assert_eq(result, expected)
}

/// Lambdas used for inverted sections should be considered truthy.
#[test]
pub fn inverted_section () {
	let template = "<{{^lambda}}{{static}}{{/lambda}}>";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".static","static");
	let result = engine.render(ctx);
	let expected = String::from("<>");
	assert_eq(result, expected)
}
}