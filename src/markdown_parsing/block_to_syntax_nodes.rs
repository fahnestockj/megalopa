use super::syntax_node::{NodeType, SyntaxNode};
pub fn block_to_syntax_nodes(block: &str) -> Vec<SyntaxNode> {
    // this checks the relevant chars at the start of the block
    if block.starts_with("#") {
        // take the #s into the nodes content and make inline nodes children
        let mut last_hashtag_idx: usize = 0;
        for (idx, char) in block.char_indices() {
            if char == '#' {
                last_hashtag_idx = idx;
            } else {
                break;
            }
        }
        assert!(last_hashtag_idx < 7);

        let rest_of_block = &block[(last_hashtag_idx + 1)..].trim_start();
        let child_nodes = str_to_inline_syntax_node(rest_of_block);
        let parent_node = SyntaxNode {
            node_type: NodeType::Heading,
            content: Some(String::from(&block[0..(last_hashtag_idx + 1)])),
            children: Box::new(child_nodes),
        };
        return vec![parent_node];
    } else if block.trim().starts_with(">") {
        let rest_of_block = block.trim_start().strip_prefix('>').unwrap();
        let child_nodes = str_to_inline_syntax_node(rest_of_block);
        let parent_node = SyntaxNode {
            node_type: NodeType::Blockquote,
            content: None,
            children: Box::new(child_nodes),
        };
        return vec![parent_node];
    } else if block.trim().starts_with("- ") {
        // break into list items and call str_to_inline_syntax_node on each
        let mut list_item_nodes: Vec<SyntaxNode> = vec![];
        for list_item in block.lines() {
            // If there is whitespace in front of the list item then nest the list
            let is_nested = list_item.starts_with("  ");
            let rest_of_list_item = list_item
                .trim_start()
                .strip_prefix("-")
                .unwrap()
                .trim_start();
            let children_of_line_item = str_to_inline_syntax_node(rest_of_list_item);

            if is_nested {
                let list_item_node = SyntaxNode {
                    content: None,
                    node_type: NodeType::ListItem,
                    children: Box::new(children_of_line_item),
                };
                let nested_list = SyntaxNode {
                    content: None,
                    children: Box::new(vec![list_item_node]),
                    node_type: NodeType::UnorderedList,
                };
                list_item_nodes.push(nested_list)
            } else {
                let list_item_node = SyntaxNode {
                    content: None,
                    node_type: NodeType::ListItem,
                    children: Box::new(children_of_line_item),
                };
                list_item_nodes.push(list_item_node)
            }
        }

        let unordered_list_node = SyntaxNode {
            content: None,
            children: Box::new(list_item_nodes),
            node_type: NodeType::UnorderedList,
        };
        return vec![unordered_list_node];
    } else if block.trim().starts_with("1.") {
        let mut list_item_nodes: Vec<SyntaxNode> = vec![];
        for list_item in block.lines() {
            let rest_of_list_item = list_item.trim()[2..].trim();
            let children_of_line_item = str_to_inline_syntax_node(rest_of_list_item);

            let list_item_node = SyntaxNode {
                content: None,
                node_type: NodeType::ListItem,
                children: Box::new(children_of_line_item),
            };
            list_item_nodes.push(list_item_node)
        }

        let ordered_list_node = SyntaxNode {
            content: None,
            children: Box::new(list_item_nodes),
            node_type: NodeType::OrderedList,
        };
        return vec![ordered_list_node];
    } else {
        return str_to_inline_syntax_node(block);
    }
}

// recursively builds the syntax nodes contained within a string
// for example bold node with child italic node with child text node
fn str_to_inline_syntax_node(string: &str) -> Vec<SyntaxNode> {
    let mut nodes: Vec<SyntaxNode> = vec![];
    let mut char_iter = string.char_indices();

    let mut text_node_contents = String::new();
    while let Some((idx, char)) = char_iter.next() {
        match char {
            '`' => {
                // end prev text node if there's content add it to Syntax Node
                if text_node_contents.chars().count() > 0 {
                    nodes.push(SyntaxNode {
                        children: Box::new(vec![]),
                        content: Some(text_node_contents.clone()),
                        node_type: NodeType::Text,
                    });
                    text_node_contents.clear();
                }

                // pull slice till closing tag
                // create syntax node and recurse on child string slice
                let idx_after_first_backtick = idx + 1;
                let offset_to_next_backtick = &string[idx_after_first_backtick..]
                    .chars()
                    .position(|c| c == '`')
                    .expect("No closing ` char found");
                let idx_after_next_backtick = idx_after_first_backtick + offset_to_next_backtick;

                // slice out the ` chars
                let sub_str = &string[idx_after_first_backtick..(idx_after_next_backtick)];
                let children = str_to_inline_syntax_node(sub_str);
                let node = SyntaxNode {
                    node_type: NodeType::Code,
                    children: Box::new(children),
                    content: None,
                };
                nodes.push(node);
                // skip chars till the closing_char_idx
                char_iter.nth(*offset_to_next_backtick);
            }
            '*' => {
                // end prev text node if there's content add it to Syntax Node
                if text_node_contents.chars().count() > 0 {
                    nodes.push(SyntaxNode {
                        children: Box::new(vec![]),
                        content: Some(text_node_contents.clone()),
                        node_type: NodeType::Text,
                    });
                    text_node_contents.clear();
                }

                let mut is_bold = false;

                // check for bold
                // pull slice till closing tag
                // create syntax node and recurse on child string slice
                let mut idx_after_opening_chars = idx + 1;
                if &string[idx_after_opening_chars..idx_after_opening_chars + 1] == "*" {
                    idx_after_opening_chars += 1;
                    is_bold = true;
                }

                let mut offset_to_closing_chars: usize;
                if is_bold {
                    let byte_idx_of_closing_chars = string[idx_after_opening_chars..]
                        .find("**")
                        .expect("no closing ** chars found");
                    let offset_to_start_of_closing_chars = string[idx_after_opening_chars..]
                        [..byte_idx_of_closing_chars]
                        .chars()
                        .count();
                    offset_to_closing_chars = offset_to_start_of_closing_chars;
                } else {
                    offset_to_closing_chars = string[idx_after_opening_chars..]
                        .chars()
                        .position(|c| c == '*')
                        .expect("No closing * char found");
                }
                let idx_after_next_asterisk = idx_after_opening_chars + offset_to_closing_chars;

                // slice out the * or ** chars
                let sub_str = &string[idx_after_opening_chars..(idx_after_next_asterisk)];
                let children = str_to_inline_syntax_node(sub_str);
                let node = SyntaxNode {
                    node_type: NodeType::Italic,
                    children: Box::new(children),
                    content: None,
                };
                nodes.push(node);
                // skip chars till the closing_char_idx
                //TODO: bold needs to move 2 more than italics for some reason
                if is_bold {
                    offset_to_closing_chars += 2;
                }

                char_iter.nth(offset_to_closing_chars);
            }
            _ => text_node_contents.push(char),
        }
    }
    // check for remaining text
    if text_node_contents.chars().count() > 0 {
        nodes.push(SyntaxNode {
            children: Box::new(vec![]),
            content: Some(text_node_contents.clone()),
            node_type: NodeType::Text,
        });
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn testing_bold() {
        let bold = "why **god** why **god** ";
        let nodes = str_to_inline_syntax_node(&bold);
        assert_eq!(nodes.iter().count(), 5)
    }
    #[test]
    pub fn testing_italics() {
        let italics = "why *god* why *god* ";
        let nodes = str_to_inline_syntax_node(&italics);
        assert_eq!(nodes.iter().count(), 5)
    }
    #[test]
    pub fn testing_backticks() {
        let backticks = "why `god` why `god` ";
        let nodes = str_to_inline_syntax_node(&backticks);
        assert_eq!(nodes.iter().count(), 5)
    }
    #[test]
    pub fn inline_code() {
        // test for basic node type identification
        let code_block: &str = "`Hello World`";
        let code_nodes = block_to_syntax_nodes(code_block);

        let fixture = SyntaxNode {
            content: None,
            node_type: NodeType::Code,
            children: Box::new(vec![SyntaxNode {
                node_type: NodeType::Text,
                children: Box::new(vec![]),
                content: Some(String::from("Hello World")),
            }]),
        };
        assert_eq!(code_nodes[0], fixture);

        let nested_header_block = "## `Hello World`";
        let header_nodes = block_to_syntax_nodes(nested_header_block);
        let fixture_header_node = SyntaxNode {
            content: Some(String::from("##")),
            children: Box::new(vec![fixture.clone()]),
            node_type: NodeType::Heading,
        };
        assert_eq!(header_nodes[0], fixture_header_node);
    }
    #[test]
    pub fn basic_header() {
        let header: &str = "# Hello World";
        let header_nodes = block_to_syntax_nodes(header);

        let fixture = SyntaxNode {
            content: Some(String::from("#")),
            node_type: NodeType::Heading,
            children: Box::new(vec![SyntaxNode {
                node_type: NodeType::Text,
                children: Box::new(vec![]),
                content: Some(String::from("Hello World")),
            }]),
        };
        assert_eq!(header_nodes[0], fixture);
    }

    #[test]
    pub fn double_inline_code() {
        let header: &str = "`code` `code`";
        let header_nodes = block_to_syntax_nodes(header);
        assert_eq!(3, header_nodes.iter().count());

        let another = "nesting `code` in a `block`";
        let header_nodes = block_to_syntax_nodes(another);
        assert_eq!(4, header_nodes.iter().count());
    }

    #[test]
    pub fn code_in_header() {
        let header: &str = "## `code` chars inbetween `code`";
        let header_nodes = block_to_syntax_nodes(header);
        assert!(header_nodes.iter().count() == 1);
    }

    #[test]
    pub fn blockquote() {
        // test for basic node type identification
        let blockquote: &str = ">Hello World";
        let blockquote_nodes = block_to_syntax_nodes(blockquote);

        let fixture = SyntaxNode {
            content: None,
            node_type: NodeType::Blockquote,
            children: Box::new(vec![SyntaxNode {
                node_type: NodeType::Text,
                children: Box::new(vec![]),
                content: Some(String::from("Hello World")),
            }]),
        };
        assert_eq!(blockquote_nodes[0], fixture);
    }
    #[test]
    pub fn lists() {
        let unordered_block = "- another list item\n- another list item\n- another list item";
        let unordered_nodes = block_to_syntax_nodes(unordered_block);
        let list_item_node = SyntaxNode {
            children: Box::new(vec![SyntaxNode {
                children: Box::new(vec![]),
                content: Some(String::from("another list item")),
                node_type: NodeType::Text,
            }]),
            node_type: NodeType::ListItem,
            content: None,
        };

        let unordered_list_fixture = SyntaxNode {
            content: None,
            children: Box::new(vec![
                list_item_node.clone(),
                list_item_node.clone(),
                list_item_node.clone(),
            ]),
            node_type: NodeType::UnorderedList,
        };

        assert_eq!(unordered_nodes[0], unordered_list_fixture);

        let ordered_block = "1. another list item\n2. another list item\n3. another list item";
        let ordered_nodes = block_to_syntax_nodes(ordered_block);

        let ordered_list_fixture = SyntaxNode {
            content: None,
            children: Box::new(vec![
                list_item_node.clone(),
                list_item_node.clone(),
                list_item_node.clone(),
            ]),
            node_type: NodeType::OrderedList,
        };
        assert_eq!(ordered_nodes[0], ordered_list_fixture);
    }

    #[test]
    pub fn nested_list() {
        let unordered_block = "- another list item\n  - another list item\n- another list item";
        let unordered_nodes = block_to_syntax_nodes(unordered_block);
        let list_item_node = SyntaxNode {
            children: Box::new(vec![SyntaxNode {
                children: Box::new(vec![]),
                content: Some(String::from("another list item")),
                node_type: NodeType::Text,
            }]),
            node_type: NodeType::ListItem,
            content: None,
        };

        let nested_list_node = SyntaxNode {
            children: Box::new(vec![list_item_node.clone()]),
            node_type: NodeType::UnorderedList,
            content: None,
        };

        let unordered_list_fixture = SyntaxNode {
            content: None,
            children: Box::new(vec![
                list_item_node.clone(),
                nested_list_node.clone(),
                list_item_node.clone(),
            ]),
            node_type: NodeType::UnorderedList,
        };

        assert_eq!(unordered_nodes[0], unordered_list_fixture);
    }
}
