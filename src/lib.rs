pub fn replace_home(path: &str) -> String {
    let home_dir_path = dirs::home_dir();
    if home_dir_path.is_none() {
        return path.to_owned();
    }
    let home_dir_path_uw = home_dir_path.unwrap();
    let home_dir = home_dir_path_uw.to_str().unwrap_or("");
    match path.get(..home_dir.len()) {
        Some(sub_path) if sub_path == home_dir => {
            format!("~{}", path.get(home_dir.len()..path.len()).unwrap_or(""))
        }
        _ => path.to_owned(),
    }
}

/// Given a path, a target length and a truncation length, returns a truncated
/// path.
///
/// `target_len` is the ideal max length of the path
/// `trunc_len` is the how many characters to include when truncating an element
/// of a path
pub fn truncate_path(path: &str, target_len: usize, trunc_len: usize) -> String {
    if path.len() <= target_len {
        return path.to_owned();
    }
    let mut out_char_count = path.chars().count();
    // Preallocate `out_len` in case truncation is not possible
    let mut out = String::with_capacity(path.len());

    let mut components = path.split('/').peekable();
    while let Some(dir) = components.next() {
        // Never truncate current directory
        if components.peek().is_none() {
            out.push_str(dir);
        // No truncation necessary or possible
        } else if out_char_count <= target_len || dir.len() <= trunc_len {
            out.push_str(dir);
            out.push('/');
        // Truncate
        } else {
            let adj_trun_len = match dir.chars().next() {
                Some('.') => trunc_len + 1,
                _ => trunc_len,
            };
            out.push_str(&dir.chars().take(adj_trun_len).collect::<String>());
            out.push('/');
            out_char_count -= dir.chars().count() - adj_trun_len;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_home() {
        assert_eq!(
            replace_home(&format!(
                "{}/git/vinoteca/vinoteca/node_modules",
                dirs::home_dir().unwrap().to_str().unwrap_or("")
            )),
            "~/git/vinoteca/vinoteca/node_modules"
        );
    }

    #[test]
    fn test_replace_home_outside_home() {
        assert_eq!(
            replace_home("/usr/lib/llvm/include"),
            "/usr/lib/llvm/include"
        );
    }

    #[test]
    fn test_truncated_path() {
        assert_eq!(
            truncate_path("/usr/lib/llvm/include", 1, 1),
            "/u/l/l/include"
        );
    }

    #[test]
    fn test_root() {
        assert_eq!(truncate_path("/", 1, 1), "/");
    }

    #[test]
    fn test_home() {
        assert_eq!(truncate_path("~", 1, 1), "~");
    }

    #[test]
    fn test_truncate_some_but_not_others() {
        assert_eq!(
            truncate_path("/usr/lib/llvm/include", 18, 1),
            "/u/l/llvm/include"
        );
    }

    #[test]
    fn test_different_trunc_len() {
        assert_eq!(
            truncate_path("~/git/vinoteca/vinoteca/node_modules", 1, 2),
            "~/gi/vi/vi/node_modules"
        );
    }

    #[test]
    fn test_private_dirs_get_extra_letter() {
        assert_eq!(truncate_path("~/.config/i3", 1, 1), "~/.c/i3");
    }

    #[test]
    fn test_unicode() {
        assert_eq!(
            truncate_path("~/españa/Δελτα/française", 1, 1),
            "~/e/Δ/française"
        );
        assert_eq!(truncate_path("~/문서/대학", 1, 1), "~/문/대학");
    }
}
