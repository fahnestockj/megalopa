pub struct LinkOrImageProps {
    pub name: String,
    pub href: String,
}
/// returns char length of the parsed link
pub fn parse_md_link_or_image(str: &str) -> Option<(LinkOrImageProps, usize)> {
    let mut iter = str.chars();
    if !iter.next().eq(&Some('[')) {
        return None;
    }

    let closing_bracket_char_idx = iter.clone().position(|char| char.eq(&']'))?;
    let name: String = iter.clone().take(closing_bracket_char_idx).collect();
    iter.nth(closing_bracket_char_idx);
    if !iter.next().eq(&Some('(')) {
        return None;
    }

    let closing_parenthesis_char_idx = iter.clone().position(|char| char.eq(&')'))?;
    let href: String = iter.clone().take(closing_parenthesis_char_idx).collect();
    iter.nth(closing_parenthesis_char_idx);

    let link_char_len: usize = href.chars().count() + name.chars().count() + 4;
    Some((LinkOrImageProps { href, name }, link_char_len))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_link() {
        let link = "[hi](/hi)";

        let res = parse_md_link_or_image(link);
        assert!(res.is_some());

        if let Some(res) = res {
            assert_eq!(res.0.href, "/hi");
            assert_eq!(res.0.name, "hi");
            assert_eq!(res.1, link.chars().count())
        }
    }
    #[test]
    pub fn with_longer_str() {
        let link = "[text](/text) and then more text";

        let res = parse_md_link_or_image(link);
        assert!(res.is_some());

        if let Some(res) = res {
            assert_eq!(res.0.href, "/text");
            assert_eq!(res.0.name, "text");
            assert_eq!(res.1, 13)
        }
    }
}
