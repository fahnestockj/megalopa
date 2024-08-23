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
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // pub fn md_to_html_test() {
    //     let md = "# hi\nhello\n- list item\n- list item\nnot\n1. list item\n2. list item";
    //     let html = parse_markdown(md);
    //     let html_fixture = "<div><h1>hi</h1>hello<ul><li>list item</li><li>list item</li></ul><ol><li>list item</li><li>list item</li></ol></div>";
    //     assert_eq!(html, html_fixture);
    // }

    #[test]
    pub fn testing() {
        let md = "
        # Hello World

- I am a list
- of items

`I am code`

## subheading 

> inline block

> # heading in inline block


nesting `code` in a `block`
";

        let html = parse_markdown(md);
        let html_fixture = "";
        assert_eq!(html, html_fixture);
    }
}
