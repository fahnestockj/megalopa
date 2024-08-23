use block_to_syntax_nodes::block_to_syntax_nodes;
use md_to_blocks::md_to_blocks;
use syntax_node::{SyntaxNode, ToHtml};

mod block_to_syntax_nodes;
mod md_to_blocks;
mod syntax_node;

pub fn parse_markdown(md_content: &str) -> String {
    let mut blocks = md_to_blocks(md_content);
    let syntax_nodes: Vec<SyntaxNode> = blocks
        .iter_mut()
        .flat_map(|block| block_to_syntax_nodes(block))
        .collect();

    let mut html = String::from("<div>");

    syntax_nodes
        .iter()
        .for_each(|node| html.push_str(node.to_html().as_str()));
    html.push_str("</div>");
    return html;
}
