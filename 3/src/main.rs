//Given a string of words, implement a function that returns the shortest word in the string

fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let text = "Given a string of words, implement a function that returns the shortest word in the string";
    match shortest_word(&text) {
        Some(word) => println!("The shortest word is: '{}'", word),
        None => println!("The input string is empty or contains no words."),
    }
}
