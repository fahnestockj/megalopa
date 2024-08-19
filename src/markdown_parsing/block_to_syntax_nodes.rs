

// blocks of text can contain layers of nodes
// should it be a vec or should it be a single Node
// why do an intermediate representation, let's just pull what we need
pub fn block_to_syntax_node(block: &str) -> SyntaxNode {
    // look for identifiers from the list...
    let block = block.clone().trim();
    if block.starts_with("#") {
        // find as many # and set that heading level
        let mut h_type = 1;
        for char in block.chars() {
            if char == '#' {
                h_type = h_type + 1;
            } else {
                break;
            }
        }
        // 
    }
    if block.starts_with("`") {}
    if block.starts_with("-") {}
    if block.starts_with("1.") {}
    // just text node
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn types_of_nodes() {
        // test for basic node type identification
        let block: &str = "# Hello World";
        let syntax_nodes = block_to_syntax_node(block);
        // assert_eq!(text_nodes, vec![]);
    }
}