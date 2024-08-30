use regex;
pub fn md_to_blocks(md_file: &str) -> Vec<String> {
    let mut blocks: Vec<String> = vec![];

    let mut unordered_list_block = String::new();
    let mut ordered_list_block = String::new();
    let ordered_list_regex = regex::Regex::new(r"^[1-9][1-9]?\.").unwrap();

    let mut lines_itr = md_file.lines();
    while let Some(mut line) = lines_itr.next() {
        if line.starts_with("---") && blocks.len() == 0 {
            //skip frontmatter
            while let Some(line) = lines_itr.next() {
                if line.trim().starts_with("---") {
                    break;
                }
            }
            line = lines_itr
                .next()
                .expect("Md file is empty other than frontmatter");
        }

        // We ONLY add blocks when
        // 1. We find a text block we add it right away (and any existing list blocks which get cleared)
        // 2. We find a list block that's new! Then if the other type of list exists we add it as a block and clear the var

        if line.trim().starts_with("- ") {
            if !unordered_list_block.is_empty() {
                unordered_list_block.push_str(format!("\n{}", line).as_str());
            } else {
                // new list! check for an existing ordered list and push it
                if !ordered_list_block.is_empty() {
                    blocks.push(ordered_list_block.clone());
                    ordered_list_block.clear();
                }
                unordered_list_block = line.to_string();
            }
        }
        // this needs to be number agnostic
        else if ordered_list_regex.is_match(line.trim()) {
            if !ordered_list_block.is_empty() {
                ordered_list_block.push_str(format!("\n{}", line).as_str());
            } else {
                // new list! check for an existing unordered list and push it
                if !unordered_list_block.is_empty() {
                    blocks.push(unordered_list_block.clone());
                    unordered_list_block.clear();
                }
                ordered_list_block = line.to_string();
            }
        } else {
            // regular text block
            // check if a list exists and needs to be pushed
            if !unordered_list_block.is_empty() {
                blocks.push(unordered_list_block.clone());
                unordered_list_block.clear();
            }

            if !ordered_list_block.is_empty() {
                blocks.push(ordered_list_block.clone());
                ordered_list_block.clear();
            }
            blocks.push(line.to_string());
        }
    }
    // check for remaining blocks
    if !unordered_list_block.is_empty() {
        blocks.push(unordered_list_block.clone())
    }
    if !ordered_list_block.is_empty() {
        blocks.push(ordered_list_block.clone())
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn text_block_test() {
        let md = "text\ntext\ntext";
        let blocks = md_to_blocks(md);

        let mut str_fixture = vec!["text", "text", "text"];
        let string_fixture: Vec<String> =
            str_fixture.iter_mut().map(|str| str.to_string()).collect();

        assert_eq!(blocks, string_fixture);
    }

    #[test]
    pub fn block_test() {
        let md = "# hi\nhello\n- list item\n- list item\nnot\n1. list item\n2. list item";
        let blocks = md_to_blocks(md);

        let mut str_fixture = vec![
            "# hi",
            "hello",
            "- list item\n- list item",
            "not",
            "1. list item\n2. list item",
        ];
        let string_fixture: Vec<String> =
            str_fixture.iter_mut().map(|str| str.to_string()).collect();

        assert_eq!(blocks, string_fixture);
    }
}
