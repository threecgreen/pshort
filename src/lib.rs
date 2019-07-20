pub fn truncate_path(path: &str, len: usize, delimiter: char) -> String {
    let mut truncated = "".to_owned();
    // if path.get(0..1) == Some("~") {
    //     truncated.push_str("~/");
    // } else {
    //     truncated.push('/');
    // }
    for dir in path.split('/') {
        match dir.get(0..len) {
            Some(truncated_dir) => truncated.push_str(truncated_dir),
            None => truncated.push_str(dir),
        };
        truncated.push(delimiter);
    }
    return truncated;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncated_path() {
        assert_eq!(truncate_path("/usr/lib/llvm/include", 1, '/'), "/u/l/l/include");
    }
}
