pub fn md_to_blocks(md_file: &str) -> Vec<String> {
    let mut blocks: Vec<String> = vec![];

    let mut lines_itr = md_file.lines();
    while let Some(mut line) = lines_itr.next() {
        //TODO: this got messy rewrite with normal vecs and indexes instead of iterators
        loop {
            let mut list_block = String::new();
            if line.trim().starts_with("- ") {
                list_block = String::from(line);
                while let Some(nested_line) = lines_itr.next() {
                    if nested_line.trim().starts_with("- ") {
                        list_block.push_str(nested_line);
                    } else {
                        line = nested_line;
                        break;
                    }
                }
            }
            if line.trim().starts_with("1. ") {
                list_block = String::from(line);
                let mut ordered_list_num = 2;
                while let Some(nested_line) = lines_itr.next() {
                    let prefix = format!("{}.", ordered_list_num);
                    if nested_line.trim().starts_with(&prefix) {
                        list_block.push_str(nested_line);
                        ordered_list_num += 1;
                    } else {
                        line = nested_line;
                        break;
                    }
                }
            }

            if list_block.chars().count() == 0 {
                blocks.push(String::from(line));
                break;
            } else {
                blocks.push(list_block);
            }
        }
    }

    todo!()
}



