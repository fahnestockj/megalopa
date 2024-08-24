#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SyntaxNode {
    pub content: Option<String>,
    pub node_type: NodeType,
    pub children: Box<Vec<SyntaxNode>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NodeType {
    Text,
    Heading,
    Code,
    UnorderedList,
    ListItem,
    OrderedList,
    Blockquote,
}

pub trait ToHtml {
    fn to_html(&self) -> String;
}

impl ToHtml for SyntaxNode {
    fn to_html(&self) -> String {
        match self.node_type {
            NodeType::Text => {
                // Nothing just return contents
                self.content
                    .as_ref()
                    .expect("Text node should have content")
                    .to_string()
            }
            NodeType::Code => {
                // wrap with <code> block
                let mut wrapped_contents = String::from("<code>");
                self.children
                    .iter()
                    .for_each(|child| wrapped_contents.push_str(child.to_html().as_str()));
                wrapped_contents.push_str("</code>");
                wrapped_contents
            }
            NodeType::Heading => {
                // find heading num
                let mut header_count = 0;
                for char in self
                    .content
                    .as_ref()
                    .expect("Heading should have content")
                    .chars()
                {
                    if char == '#' {
                        header_count += 1;
                    }
                }
                assert!(header_count > 0);
                assert!(header_count < 7);
                let mut header_tag = String::from("<h");
                header_tag.push_str(&header_count.to_string());
                header_tag.push_str(">");
                let mut wrapped_contents = header_tag.clone();

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
                    .all(|child| { child.node_type == NodeType::ListItem }));

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
                    .all(|child| { child.node_type == NodeType::ListItem }));

                let mut wrapped_contents = String::from("<ul>");
                self.children
                    .iter()
                    .for_each(|child| wrapped_contents.push_str(&child.to_html()));
                wrapped_contents.push_str("</ul>");
                wrapped_contents
            }
            NodeType::ListItem => {
                // number is already removed so wrap with <li>
                let mut wrapped_contents = String::from("<li>");
                self.children
                    .iter()
                    .for_each(|child| wrapped_contents.push_str(&child.to_html()));
                wrapped_contents.push_str("</li>");
                wrapped_contents
            }
            NodeType::Blockquote => {
                let mut wrapped_contents = String::from("<blockquote>");
                self.children
                    .iter()
                    .for_each(|child| wrapped_contents.push_str(&child.to_html()));
                wrapped_contents.push_str("</blockquote>");
                wrapped_contents
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    pub fn heading_to_html() {
        let node: SyntaxNode = SyntaxNode {
            content: Some(String::from("#")),
            children: Box::new(vec![SyntaxNode {
                content: Some(String::from("heading")),
                children: Box::default(),
                node_type: NodeType::Text,
            }]),
            node_type: NodeType::Heading,
        };

        assert_eq!(node.to_html(), "<h1>heading</h1>");
    }
    #[test]
    pub fn code_to_html() {
        let node: SyntaxNode = SyntaxNode {
            content: None,
            children: Box::new(vec![SyntaxNode {
                content: Some(String::from("code")),
                children: Box::default(),
                node_type: NodeType::Text,
            }]),
            node_type: NodeType::Code,
        };

        assert_eq!(node.to_html(), "<code>code</code>");
    }

    #[test]
    pub fn list_to_html() {
        let list_item_node = SyntaxNode {
            content: None,
            children: Box::new(vec![SyntaxNode {
                content: Some(String::from("item content")),
                children: Box::default(),
                node_type: NodeType::Text,
            }]),
            node_type: NodeType::ListItem,
        };

        let node = SyntaxNode {
            content: None,
            children: Box::new(vec![list_item_node.clone(), list_item_node.clone()]),
            node_type: NodeType::UnorderedList,
        };

        assert_eq!(node.to_html(), "<ul><li>item content</li><li>item content</li></ul>");
    }
}
