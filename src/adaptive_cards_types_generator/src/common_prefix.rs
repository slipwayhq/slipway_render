pub(super) fn common_prefix(strings: &[String]) -> String {
    if strings.len() <= 1 {
        return String::new();
    }

    let mut prefix = strings[0].clone();
    for s in strings.iter().skip(1) {
        while !s.starts_with(&prefix) {
            if prefix.is_empty() {
                return String::new();
            }
            prefix.pop();
        }
    }

    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_find_common_prefix() {
        let strings = vec![
            "Action.OpenUrl".to_string(),
            "Action.Submit".to_string(),
            "Action.ShowCard".to_string(),
        ];
        assert_eq!("Action.", common_prefix(&strings));
    }

    #[test]
    fn it_should_find_empty_common_prefix_with_empty_string() {
        let strings = vec!["".to_string(), "Action.Submit".to_string()];
        assert_eq!("", common_prefix(&strings));
    }

    #[test]
    fn it_should_find_empty_common_prefix_with_no_common_prefix() {
        let strings = vec!["Action.OpenUrl".to_string(), "Submit".to_string()];
        assert_eq!("", common_prefix(&strings));
    }

    #[test]
    fn it_should_find_empty_common_prefix_with_single_string() {
        let strings = vec!["Action.OpenUrl".to_string()];
        assert_eq!("", common_prefix(&strings));
    }
}
