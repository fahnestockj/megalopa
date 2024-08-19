#[derive(Debug, Eq, PartialEq)]
pub struct SyntaxNode {
    pub content: String,
    pub text_type: NodeType,
    pub children: Box<Vec<SyntaxNode>>,
    pub parent: Box<SyntaxNode>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum NodeType {
    Text,
    Heading,
    Code,
    UnorderedList,
    OrderedList,
    InlineQuote,
}

trait ToHtml {
    fn to_html(self) -> String;
}

impl ToHtml for SyntaxNode {
    fn to_html(self) -> String {
        match self.text_type {
            NodeType::Text => {
                // Nothing just return contents
                self.content
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
                for char in self.content.chars() {
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
                // this one will be tricky...
            }
            NodeType::UnorderedList => {}
            NodeType::InlineQuote => {}
        }
    }
}
