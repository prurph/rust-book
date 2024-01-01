use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // Failure to read immediately exits with the error instead
    // and a non-zero status
    let file_contents = fs::read_to_string("hello.txt")?;
    println!("{:?}", last_char_of_first_line(&file_contents));

    // Otherwise exit with 0 status
    Ok(())
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
