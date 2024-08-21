#[derive(Debug, Eq, PartialEq)]
pub struct SyntaxNode {
    pub content: Option<String>,
    pub node_type: NodeType,
    pub children: Box<Vec<SyntaxNode>>,
    pub parent: Option<Box<SyntaxNode>>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum NodeType {
    Text,
    Heading,
    Code,
    UnorderedList,
    UnorderdListItem,
    OrderedList,
    OrderedListItem,
    InlineQuote,
}

trait ToHtml {
    fn to_html(self) -> String;
}

impl ToHtml for SyntaxNode {
    fn to_html(self) -> String {
        match self.node_type {
            NodeType::Text => {
                // Nothing just return contents
                self.content.expect("Text node should have content")
            }
            NodeType::Code => {
                // wrap with <code> block
                let mut wrapped_contents = String::from("<code>");
                self.children
                    .iter()
                    .for_each(|child| wrapped_contents.push_str(&child.to_html()));
                wrapped_contents.push_str("</code>");
                wrapped_contents
            }
            NodeType::Heading => {
                // find heading num
                let mut header_count = 0;
                for char in self.content.expect("Heading should have content").chars() {
                    if char == '#' {
                        header_count += 1;
                    }
                }
                assert!(header_count > 0);
                assert!(header_count < 7);
                let mut header_tag = String::from("<h");
                header_tag.push_str(&header_count.to_string());
                header_tag.push_str(">");
                let mut wrapped_contents = String::from("<h");
                self.children
                    .iter()
                    .for_each(|child| wrapped_contents.push_str(&child.to_html()));

                let mut closing_tag = header_tag.clone();
                closing_tag.insert(1, '/');
                wrapped_contents.push_str(&closing_tag);
                wrapped_contents
            }
            NodeType::OrderedList => {
                // wrap contents with <ol>
                // children of this node should only be ordered list items
                assert!(self
                    .children
                    .iter()
                    .all(|child| { child.node_type == NodeType::OrderedListItem }));

                let mut wrapped_contents = String::from("<ol>");
                self.children
                    .iter()
                    .for_each(|child| wrapped_contents.push_str(&child.to_html()));
                wrapped_contents.push_str("</ol>");
                wrapped_contents
            }
            NodeType::UnorderedList => {
                // same as OrderedList just <ul>
                assert!(self
                    .children
                    .iter()
                    .all(|child| { child.node_type == NodeType::UnorderdListItem }));

                let mut wrapped_contents = String::from("<ul>");
                self.children
                    .iter()
                    .for_each(|child| wrapped_contents.push_str(&child.to_html()));
                wrapped_contents.push_str("</ul>");
                wrapped_contents
            }
            NodeType::UnorderdListItem | NodeType::OrderedListItem => {
                // number is already removed so wrap with <li>
                let mut wrapped_contents = String::from("<li>");
                self.children
                    .iter()
                    .for_each(|child| wrapped_contents.push_str(&child.to_html()));
                wrapped_contents.push_str("</li>");
                wrapped_contents
            }
            NodeType::InlineQuote => {
                let mut wrapped_contents = String::from("<q>");
                self.children
                    .iter()
                    .for_each(|child| wrapped_contents.push_str(&child.to_html()));
                wrapped_contents.push_str("</q>");
                wrapped_contents
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn to_html_test() {
        // test for basic node functionality
        let node: SyntaxNode = SyntaxNode {
            content: Some(String::from("# heading")),
            children: Box::default(),
            node_type: NodeType::Heading,
            parent: None,
        };
        assert_eq!(node.to_html(), "<h1> heading</h1>");
    }
}