pub fn find_char(string: &str, character: char) -> Option<isize> {
    string
        .chars()
        .enumerate()
        .find_map(|i, c| if c == character { Some(i) } else { None })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_char() {
        assert_eq!(find_char("Hello world", 'w'), Some(6));
        assert_eq!(find_char("Hello world", 'z').unwrap(), None);
    }
}
