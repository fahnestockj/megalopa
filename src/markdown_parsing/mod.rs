use block_to_syntax_nodes::block_to_syntax_nodes;
use md_to_blocks::md_to_blocks;
use syntax_node::{SyntaxNode, ToHtml, NodeType};

mod block_to_syntax_nodes;
mod md_to_blocks;
mod syntax_node;
mod parse_md_link_or_image;

pub fn parse_frontmatter(md_content: &str) -> Option<String> {
    let mut lines_itr = md_content.lines();
    let mut o_frontmatter: Option<String> = None;
    if let Some(first_line) = lines_itr.next() {
        if first_line.trim().eq("---") {
            while let Some(frontmatter_line) = lines_itr.next() {
                if frontmatter_line.trim().eq("---") {
                    break;
                } else {
                    if let Some(frontmatter) = o_frontmatter {
                        o_frontmatter = Some(format!("{}\n{}", frontmatter, frontmatter_line));
                    } else {
                        o_frontmatter = Some(String::from(frontmatter_line));
                    }
                }
            }
        }
    }
    o_frontmatter
}

pub fn parse_markdown(md_content: &str) -> String {
    let mut blocks = md_to_blocks(md_content);
    // wrap each block with a div to create whitespace between blocks
    let div_node = SyntaxNode {
        children: Box::new(vec![]),
        content: None,
        node_type: NodeType::Div
    };
    let syntax_nodes: Vec<SyntaxNode> = blocks
        .iter_mut()
        .map(|block| block_to_syntax_nodes(block))
        .map(|mut syntax_nodes_in_block| {
            let mut block_wrapper_div = div_node.clone();
            block_wrapper_div.children.append(&mut syntax_nodes_in_block);
            block_wrapper_div
        }).collect();
    let mut html = String::from("<div>");

    syntax_nodes
        .iter()
        .for_each(|node| html.push_str(node.to_html().as_str()));
    html.push_str("</div>");
    return html;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn md_to_html_test() {
        let md = "# hi\nhello";
        let html = parse_markdown(md);
        let html_fixture = "<div><div><h1>hi</h1></div><div>hello</div></div>";
        assert_eq!(html, html_fixture);
    }

    #[test]
    pub fn list_test() {
        let md = "- unordered\n  - nested unordered";
        let html = parse_markdown(md);
        let html_fixture = "<div><div><ul><li>unordered</li><ul><li>nested unordered</li></ul></ul></div></div>";
        assert_eq!(html, html_fixture);
    }
}
