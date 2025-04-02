fn main() {
    no_string_slice();

    string_slice();
}

fn no_string_slice() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("{}", word);

    s.clear(); // this empties the String, making it equal to ""

    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}

fn string_slice() {
    let mut s = String::from("hello world");

    let word = first_word_string_slice(&s);

    // This would throw a compile-time error!
    // s.clear();
    // If we have an immutable reference to something, we cannot also take a mutable reference.
    // Because `clear` needs to truncate the String, it needs to get a mutable reference.

    // The `println!` after the call to `clear` uses the reference in `word`, so the immutable reference
    // must still be active at that point.
    println!("the first word is: {word}");
    // Rust disallows the mutable reference in `clear` and the immutable reference in `word` from existing
    // at the same time, and compilation fails.
}
fn first_word_string_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
