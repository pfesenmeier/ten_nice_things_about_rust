pub fn find_char(string: &str, character: char) -> Option<usize> {
    string
        .chars()
        .enumerate()
        .find_map(|(i, c)| if c == character { Some(i) } else { None })
}

#[test]
fn test_find_char() {
    assert_eq!(find_char("Hello world", 'w'), Some(6));
    assert_eq!(find_char("Hello world", 'z'), None);
}

#[test]
fn test_option_helpers() {
    assert!(find_char("Hello world", 'w').is_some());
}
