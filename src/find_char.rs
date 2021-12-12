pub fn find_letter(word: &str, letter: char) -> Option<usize> {
    word
        .chars()
        .enumerate()
        .find_map(|(i, c)| if c == letter { Some(i) } else { None })
}

#[test]
fn test_find_letter() {
    assert_eq!(find_letter("Hello world", 'w'), Some(6));
    assert_eq!(find_letter("Hello world", 'z'), None);
}

#[test]
fn test_option_helpers() {
    assert_eq!(find_letter("Hello world", 'w').unwrap(), 6);

    assert_eq!(find_letter("Hello world", 'w').unwrap_or(42), 6);
    assert_eq!(find_letter("Hello world", 'z').unwrap_or(42), 42);

    assert_eq!(find_letter("Hello world", 'w').map_or(42, |i| i * 2), 12);
    assert_eq!(find_letter("Hello world", 'z').map_or(42, |i| i * 2), 42);

    assert!(find_letter("Hello world", 'w').is_some());
    assert!(find_letter("Hello world", 'z').is_none());
}

#[test]
fn silly_pattern_matching() {
    let hello_world = "Hello_world";

    let eight = find_letter(&hello_world, 'r').unwrap();

    let fourty_two = match find_letter(&hello_world, 'z') {
        Some(index) => index,
        None => 42,
    };

    let pair = (2, 5);

    let (two,..) = pair;

    struct Point {
       x: usize,
       y: usize,
    }

    let point = Point { x: 1, y: 3 };

    let Point { x: one, y: three } = point;

    if let Some(six) = find_letter(&hello_world, 'w') {

        assert_eq!(fourty_two + eight + six, 56);
    }

    assert_eq!(one + two + three, 6);
}
