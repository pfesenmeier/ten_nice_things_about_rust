use std::fs::read_to_string;

#[allow(unused)]
fn get_contents(path: &str) -> std::io::Result<()> {
    let contents = read_to_string(path).unwrap_or_else(|_| "foobar".to_string());
    println!("contents: {}", contents);
    Ok(())
}
