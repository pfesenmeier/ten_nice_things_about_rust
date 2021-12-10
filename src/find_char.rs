pub fn find_char(string: &str, char: char) -> Option<isize> {
    let mut result = -1;

    for i in 0..string.len() {
       if string.as_bytes()[i] as char == char {
          result = i.try_into().unwrap();
       }
    }

    if result == -1 {
        return None;
    }

    return Some(result);
}

#[test]
fn test_find_char() {
    assert_eq!(find_char("Hello world", 'w').unwrap(), 6);
}