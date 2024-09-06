#[cfg(test)]
mod tests {
	

/// Default content should be rendered if the block isn't overridden
#[test]
pub fn default () {
	let template = "{{$title}}Default title{{/title}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("Default title\n");
	assert_eq(result, expected)
}

/// Default content renders variables
#[test]
pub fn variable () {
	let template = "{{$foo}}default {{bar}} content{{/foo}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".bar","baz");
	let result = engine.render(ctx);
	let expected = String::from("default baz content\n");
	assert_eq(result, expected)
}

/// Default content renders triple mustache variables
#[test]
pub fn triple_mustache () {
	let template = "{{$foo}}default {{{bar}}} content{{/foo}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".bar","<baz>");
	let result = engine.render(ctx);
	let expected = String::from("default <baz> content\n");
	assert_eq(result, expected)
}

/// Default content renders sections
#[test]
pub fn sections () {
	let template = "{{$foo}}default {{#bar}}{{baz}}{{/bar}} content{{/foo}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("default qux content\n");
	assert_eq(result, expected)
}

/// Default content renders negative sections
#[test]
pub fn negative_sections () {
	let template = "{{$foo}}default {{^bar}}{{baz}}{{/bar}} content{{/foo}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".baz","three");
	let result = engine.render(ctx);
	let expected = String::from("default three content\n");
	assert_eq(result, expected)
}

/// Mustache injection in default content
#[test]
pub fn mustache_injection () {
	let template = "{{$foo}}default {{#bar}}{{baz}}{{/bar}} content{{/foo}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("default {{qux}} content\n");
	assert_eq(result, expected)
}

/// Default content rendered inside inherited templates
#[test]
pub fn inherit () {
	let template = "{{<include}}{{/include}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("default content");
	assert_eq(result, expected)
}

/// Overridden content
#[test]
pub fn overridden_content () {
	let template = "{{<super}}{{$title}}sub template title{{/title}}{{/super}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("...sub template title...");
	assert_eq(result, expected)
}

/// Context does not override argument passed into parent
#[test]
pub fn data_does_not_override_block () {
	let template = "{{<include}}{{$var}}var in template{{/var}}{{/include}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".var","var in data");
	let result = engine.render(ctx);
	let expected = String::from("var in template");
	assert_eq(result, expected)
}

/// Context does not override default content of block
#[test]
pub fn data_does_not_override_block_default () {
	let template = "{{<include}}{{/include}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".var","var in data");
	let result = engine.render(ctx);
	let expected = String::from("var in include");
	assert_eq(result, expected)
}

/// Overridden parent
#[test]
pub fn overridden_parent () {
	let template = "test {{<parent}}{{$stuff}}override{{/stuff}}{{/parent}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("test override");
	assert_eq(result, expected)
}

/// Two overridden parents with different content
#[test]
pub fn two_overridden_parents () {
	let template = "test {{<parent}}{{$stuff}}override1{{/stuff}}{{/parent}} {{<parent}}{{$stuff}}override2{{/stuff}}{{/parent}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("test |override1 default| |override2 default|\n");
	assert_eq(result, expected)
}

/// Override parent with newlines
#[test]
pub fn override_parent_with_newlines () {
	let template = "{{<parent}}{{$ballmer}}\npeaked\n\n:(\n{{/ballmer}}{{/parent}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("peaked\n\n:(\n");
	assert_eq(result, expected)
}

/// Inherit indentation when overriding a parent
#[test]
pub fn inherit_indentation () {
	let template = "{{<parent}}{{$nineties}}hammer time{{/nineties}}{{/parent}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("stop:\n  hammer time\n");
	assert_eq(result, expected)
}

/// Override one parameter but not the other
#[test]
pub fn only_one_override () {
	let template = "{{<parent}}{{$stuff2}}override two{{/stuff2}}{{/parent}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("new default one, override two");
	assert_eq(result, expected)
}

/// Parent templates behave identically to partials when called with no parameters
#[test]
pub fn parent_template () {
	let template = "{{>parent}}|{{<parent}}{{/parent}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("default content|default content");
	assert_eq(result, expected)
}

/// Recursion in inherited templates
#[test]
pub fn recursion () {
	let template = "{{<parent}}{{$foo}}override{{/foo}}{{/parent}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("override override override don't recurse");
	assert_eq(result, expected)
}

/// Top-level substitutions take precedence in multi-level inheritance
#[test]
pub fn multilevel_inheritance () {
	let template = "{{<parent}}{{$a}}c{{/a}}{{/parent}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("c");
	assert_eq(result, expected)
}

/// Top-level substitutions take precedence in multi-level inheritance
#[test]
pub fn multilevel_inheritance_no_sub_child () {
	let template = "{{<parent}}{{/parent}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("p");
	assert_eq(result, expected)
}

/// Ignores text inside parent templates, but does parse $ tags
#[test]
pub fn text_inside_parent () {
	let template = "{{<parent}} asdfasd {{$foo}}hmm{{/foo}} asdfasdfasdf {{/parent}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("hmm");
	assert_eq(result, expected)
}

/// Allows text inside a parent tag, but ignores it
#[test]
pub fn text_inside_parent () {
	let template = "{{<parent}} asdfasd asdfasdfasdf {{/parent}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("default content");
	assert_eq(result, expected)
}

/// Scope of a substituted block is evaluated in the context of the parent template
#[test]
pub fn block_scope () {
	let template = "{{<parent}}{{$block}}I say {{fruit}}.{{/block}}{{/parent}}";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	ctx.insert(".fruit","apples");
	let result = engine.render(ctx);
	let expected = String::from("I say bananas.");
	assert_eq(result, expected)
}

/// A parent's opening and closing tags need not be on separate lines in order to be standalone
#[test]
pub fn standalone_parent () {
	let template = "Hi,\n  {{<parent}}{{/parent}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("Hi,\n  one\n  two\n");
	assert_eq(result, expected)
}

/// A block's opening and closing tags need not be on separate lines in order to be standalone
#[test]
pub fn standalone_block () {
	let template = "{{<parent}}{{$block}}\none\ntwo{{/block}}\n{{/parent}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("Hi,\n  one\n  two\n");
	assert_eq(result, expected)
}

/// Block indentation is removed at the site of definition and added at the site of expansion
#[test]
pub fn block_reindentation () {
	let template = "{{<parent}}{{$block}}\n    one\n    two\n{{/block}}{{/parent}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("Hi,\n  one\n  two\n");
	assert_eq(result, expected)
}

/// When the block opening tag is standalone, indentation is determined by default content
#[test]
pub fn intrinsic_indentation () {
	let template = "{{<parent}}{{$block}}\none\ntwo\n{{/block}}{{/parent}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("Hi,\n  one\n  two\n");
	assert_eq(result, expected)
}

/// Nested blocks are reindented relative to the surrounding block
#[test]
pub fn nested_block_reindentation () {
	let template = "{{<parent}}{{$nested}}\nthree\n{{/nested}}{{/parent}}\n";
	let engine = create_oneoff_engine(template);
	let mut ctx = std::collections::HashMap::new();
	let result = engine.render(ctx);
	let expected = String::from("one\n  three\n");
	assert_eq(result, expected)
}
}