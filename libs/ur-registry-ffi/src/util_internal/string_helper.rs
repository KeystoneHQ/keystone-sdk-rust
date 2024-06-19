pub fn remove_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    match s.strip_prefix(prefix) {
        Some(s) => s,
        None => s,
    }
}

pub fn remove_prefix_0x(s: &str) -> &str {
    remove_prefix(s, "0x")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_prefix() {
        let given_str = "k-target_data";
        let expect_str = "target_data";

        assert_eq!(expect_str, remove_prefix(given_str, "k-"));
    }

    #[test]
    fn test_remove_prefix_without_given_prefix() {
        let given_str = "target_data";
        let expect_str = "target_data";

        assert_eq!(expect_str, remove_prefix(given_str, "k-"));
    }

    #[test]
    fn test_remove_prefix_0x() {
        let given_str = "0x01234567";
        let expect_str = "01234567";

        assert_eq!(expect_str, remove_prefix_0x(given_str));
    }

    #[test]
    fn test_remove_prefix_0x_without_0x() {
        let given_str = "01234567";
        let expect_str = "01234567";

        assert_eq!(expect_str, remove_prefix_0x(given_str));
    }
}
