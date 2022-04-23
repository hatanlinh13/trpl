fn main() {
    let string = String::from("Hello, world!");
    let first = first_word(&string);
    println!("The first word is: {}", first);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
