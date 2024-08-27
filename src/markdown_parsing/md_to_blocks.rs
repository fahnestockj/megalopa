pub fn md_to_blocks(md_file: &str) -> Vec<String> {
    let mut blocks: Vec<String> = vec![];

    let mut lines_itr = md_file.lines();
    while let Some(mut line) = lines_itr.next() {
        if line.trim().starts_with("---") {
            //skip frontmatter
            while let Some(mut line) = lines_itr.next() {
                if line.trim().starts_with("---") {
                    break;
                }
            }
            line = lines_itr
                .next()
                .expect("Md file is empty other than frontmatter");
        }

        // TODO: cleanup
        loop {
            let mut list_block = String::new();
            if line.trim().starts_with("- ") {
                list_block = String::from(line);
                while let Some(nested_line) = lines_itr.next() {
                    if nested_line.trim().starts_with("- ") {
                        list_block.push_str("\n");
                        list_block.push_str(nested_line);
                    } else {
                        line = nested_line;
                        break;
                    }
                }
                blocks.push(list_block);
                list_block = String::new();
            } else if line.trim().starts_with("1. ") {
                list_block = String::from(line);
                let mut ordered_list_num = 2;
                while let Some(nested_line) = lines_itr.next() {
                    let prefix = format!("{}.", ordered_list_num);
                    if nested_line.trim().starts_with(&prefix) {
                        list_block.push_str("\n");
                        list_block.push_str(nested_line);
                        ordered_list_num += 1;
                    } else {
                        line = nested_line;
                        break;
                    }
                }
                blocks.push(list_block);
                list_block = String::new();
            } else {
                blocks.push(String::from(line));
            }

            if list_block.chars().count() == 0 {
                break;
            }
        }
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn block_test() {
        let md = "# hi\nhello\n- list item\n- list item\nnot\n1. list item\n2. list item";
        let blocks = md_to_blocks(md);

        let mut str_fixture = vec![
            "# hi",
            "hello",
            "- list item\n- list item",
            "1. list item\n2. list item",
        ];
        let string_fixture: Vec<String> =
            str_fixture.iter_mut().map(|str| str.to_string()).collect();

        assert_eq!(blocks, string_fixture);
    }
}
