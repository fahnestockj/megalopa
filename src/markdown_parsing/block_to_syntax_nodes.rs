
use markdown::mdast::Node;

use super::syntax_node::{NodeType, SyntaxNode};
pub fn block_to_syntax_nodes(block: &str) -> Vec<SyntaxNode> {
    // this checks the relevant chars at the start of the block
    let block = block.trim();
    if block.starts_with("#") {
        // take the #s into the nodes content and make inline nodes children
        let mut last_hashtag_idx: usize;
        for (idx, char) in block.char_indices() {
            if char == '#' {
                last_hashtag_idx = idx;
            } else {
                break;
            }
        }
        assert!(last_hashtag_idx > 0);
        assert!(last_hashtag_idx < 7);

        let rest_of_block = &block[last_hashtag_idx..];
        let child_nodes = str_to_inline_syntax_node(rest_of_block);
        let parent_node = SyntaxNode {
            node_type: NodeType::Heading,
            content: Some(String::from(&block[0..last_hashtag_idx])),
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
    } else if block.starts_with("1.") {
    } else {
        // text node but
        // still check for inline nodes
    }
}

// recursively builds the syntax nodes contained within a string
// for example bold node with child italic node with child text node
fn str_to_inline_syntax_node(str: &str) -> Vec<SyntaxNode> {
    let mut nodes: Vec<SyntaxNode> = vec![];
    let str = str.trim();
    let mut char_iter = str.char_indices();
    while let Some((idx, char)) = char_iter.next() {
        // for (idx, char) in block.char_indices() {
        // check char for each char that can start a syntax node
        match char {
            '`' => {
                // pull slice till closing tag
                // create syntax node and recall on string slice
                let starting_idx = idx + 1;
                let closing_char_idx = &str[starting_idx..]
                    .chars()
                    .position(|c| c == '`')
                    .expect("No closing ` char found");
                // slice out the ` chars
                let sub_block = &str[starting_idx..*closing_char_idx];
                let children = block_to_syntax_nodes(sub_block);
                let node = SyntaxNode {
                    node_type: NodeType::Code,
                    children: Box::new(children),
                    content: None,
                };
                nodes.push(node);
                // skip chars till the closing_char_idx
                char_iter.nth(*closing_char_idx);
            }
            _ => {}
        }
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn inline_code() {
        // test for basic node type identification
        let block: &str = "`Hello World`";
        let syntax_nodes = block_to_syntax_nodes(block);

        let mock = SyntaxNode {
            content: None,
            node_type: NodeType::Code,
            children: Box::new(vec![]),
        };
        assert_eq!(syntax_nodes[0], mock);
    }
}
