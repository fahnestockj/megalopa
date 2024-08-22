use markdown::mdast::Node;

use super::syntax_node::{NodeType, SyntaxNode};
pub fn block_to_syntax_nodes(block: &str) -> Vec<SyntaxNode> {
    let mut nodes: Vec<SyntaxNode> = vec![];
    let block = block.clone().trim();
    let mut char_iter = block.char_indices();
    while let Some((idx, char)) = char_iter.next() {
        // for (idx, char) in block.char_indices() {
        // check char for each char that can start a syntax node
        match char {
            '`' => {
                // pull slice till closing tag
                // create syntax node and recall on string slice
                let starting_idx = idx + 1;
                let closing_char_idx = &block[starting_idx..]
                    .chars()
                    .position(|c| c == '`')
                    .expect("No closing ` char found");
                // slice out the ` chars
                let sub_block = &block[starting_idx..*closing_char_idx];
                // the idea behind the recursion is each function call is responsible for it's string slice and finding all nested nodes that aren't just text nodes
                // but only being able to return a single Syntax node is not going to be able to represent that finding several nested nodes case
                // for example one block can contain a leaf text node then an inline code block then another leaft text node
                let children = block_to_syntax_nodes(sub_block);
                let node = SyntaxNode {
                    node_type: NodeType::Code,
                    children: Box::new(children),
                    content: None,
                    parent: None,
                };
                nodes.push(node);
                // skip chars till the closing_char_idx
                char_iter.nth(*closing_char_idx);
            }
            _ => {}
            
             // block.starts_with("#")
              // block.starts_with("-")
              // block.starts_with("1.")
              // block.starts_with(">")
        }
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn types_of_nodes() {
        // test for basic node type identification
        let block: &str = "`Hello World`";
        let syntax_nodes = block_to_syntax_nodes(block);

        let mock = SyntaxNode {
            content: None,
            node_type: NodeType::Code,
            children: Box::new(vec![]),
            parent: None,
        };
        assert_eq!(syntax_nodes, vec![mock]);
    }
}
