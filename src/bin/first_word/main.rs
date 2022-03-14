use std::io;

fn main() {
    let mut words = String::new();
    println!("Please input words: ");
    io::stdin().read_line(&mut words).expect("Failed to get input");

    let word = first_word(&words);

    println!("The first word is: {}", word);

}

fn first_word(s: &str) -> &str {
    let words = s.as_bytes();

    for (i, &item) in words.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }

    }
    &s[..]
}