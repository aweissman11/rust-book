fn main() {
    println!("Hello, world!");
    let s = String::from("hello world!");
    let s1 = first_word(&s);
    println!("s1: {s1}");
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
