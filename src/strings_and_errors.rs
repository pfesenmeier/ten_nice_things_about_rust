use std::str::Utf8Error;

/// Parses bytes into string.
///    
///    ```
///    use ten_nice_things_about_rust::strings_and_errors::make_string;
///
///    let sparkle_heart = [240, 159, 146, 150];
///    let sparkle_heart = make_string(&sparkle_heart);
///    assert_eq!(sparkle_heart, Ok("💖"));
///    ```
pub fn make_string(bytes: &[u8]) -> Result<&str, Utf8Error> {
    // some bytes, in a stack-allocated array

    // We know these bytes are valid, so just use `unwrap()`.
    std::str::from_utf8(bytes)
}

#[test]
fn sparkle_heart() {
    let sparkle_heart = make_string(&[240, 159, 146, 150]);
    assert_eq!(sparkle_heart, Ok("💖"));
}

#[test]
fn not_ut8() {
    let corrupted_utf8_result = make_string(&[159, 146, 150]);
    assert!(corrupted_utf8_result.is_err());
}

#[allow(unused)]
fn handle_err_example() {
    let bad_result = make_string(&[159, 146, 150]);

    match bad_result {
       Ok(_) => todo!(),
       Err(_) => todo!(),
    }
}
