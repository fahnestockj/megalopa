use crate::markdown_parsing::syntax_node::NodeType;
use super::syntax_node::SyntaxNode;


pub fn split_text_nodes_with_links_or_images(text_node: SyntaxNode) {
  assert_eq!(text_node.node_type, NodeType::Text);
  let text = text_node.content.expect("text nodes should have content");
  
  let image_regex = regex::Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
  let link_regex = regex::Regex::new( r"\[(.*?)\]\((.*?)\)").unwrap();
  todo!()

}