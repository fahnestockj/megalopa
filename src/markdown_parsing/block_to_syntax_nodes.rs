use markdown::mdast::Node;

use super::syntax_node::{NodeType, SyntaxNode};
pub fn block_to_syntax_nodes(block: &str) -> Vec<SyntaxNode> {
    // this checks the relevant chars at the start of the block
    let block = block.trim();
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
        assert!(last_hashtag_idx > 0);
        assert!(last_hashtag_idx < 7);

        let rest_of_block = &block[(last_hashtag_idx + 1)..].trim();
        let child_nodes = str_to_inline_syntax_node(rest_of_block);
        let parent_node = SyntaxNode {
            node_type: NodeType::Heading,
            content: Some(String::from(&block[0..(last_hashtag_idx + 1)])),
            children: Box::new(child_nodes),
        };
        return vec![parent_node];
    } else if block.starts_with(">") {
        let rest_of_block = block.strip_prefix('>').unwrap();
        let child_nodes = str_to_inline_syntax_node(rest_of_block);
        let parent_node = SyntaxNode {
            node_type: NodeType::Blockquote,
            content: None,
            children: Box::new(child_nodes),
        };
        return vec![parent_node];
    } else if block.starts_with("-") {
        // break into list items and call str_to_inline_syntax_node on each
        let mut list_item_nodes: Vec<SyntaxNode> = vec![];
        for list_item in block.lines() {
            let rest_of_list_item = list_item.trim().strip_prefix("-").unwrap().trim();
            let children_of_line_item = str_to_inline_syntax_node(rest_of_list_item);

            let list_item_node = SyntaxNode {
                content: None,
                node_type: NodeType::ListItem,
                children: Box::new(children_of_line_item),
            };
            list_item_nodes.push(list_item_node)
        }

        let unordered_list_node = SyntaxNode {
            content: None,
            children: Box::new(list_item_nodes),
            node_type: NodeType::UnorderedList,
        };
        return vec![unordered_list_node];
    } else if block.starts_with("1.") {
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
fn str_to_inline_syntax_node(str: &str) -> Vec<SyntaxNode> {
    let mut nodes: Vec<SyntaxNode> = vec![];
    let str = str.trim();
    let mut char_iter = str.char_indices();

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
                let starting_idx = idx + 1;
                let closing_char_idx = &str[starting_idx..]
                    .chars()
                    .position(|c| c == '`')
                    .expect("No closing ` char found");
                // slice out the ` chars
                let sub_str = &str[starting_idx..(*closing_char_idx + 1)];
                let children = str_to_inline_syntax_node(sub_str);
                let node = SyntaxNode {
                    node_type: NodeType::Code,
                    children: Box::new(children),
                    content: None,
                };
                nodes.push(node);
                // skip chars till the closing_char_idx
                char_iter.nth(*closing_char_idx);
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
        let unordered_block = " - another list item\n - another list item\n - another list item";
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

        let ordered_block = " 1. another list item\n 2. another list item\n 3. another list item";
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
}
