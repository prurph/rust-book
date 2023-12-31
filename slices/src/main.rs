fn main() {
    let s = String::from("hello world!");

    // Callable on string slices (&str)
    let word = first_word(&s[0..6]);
    let word = first_word(&s[..]);

    // Also callable on `&String`, which gets "deref coerced" to &str
    let word = first_word(&s);

    let s_literal = "hello world";

    // Callable on slices of string literal
    let word = first_word(&s_literal[0..6]);
    let word = first_word(&s_literal[..]);

    // String literal is a slice already, so this works too
    let word = first_word(&s_literal);

    println!("The first word is: {}", first_word(&s));
}

// Take &str not &String so this can be called on string literals and `String`
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}
