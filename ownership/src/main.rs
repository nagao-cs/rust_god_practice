fn main() {
    let sentense = String::from("Rust is system programming language");
    let longest_word = longest_word(&sentense[..]);
    println!("longest word: {}", longest_word);
}

fn longest_word(s: &str) -> &str {
    let mut longest = "";

    for word in s.split_whitespace() {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    return &longest;
}
