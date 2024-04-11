#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn Pathbuf() {
        let build_dir = "public";
        let test_path = PathBuf::from(build_dir);
        assert_eq!(test_path, current_dir().unwrap());
    }
}
