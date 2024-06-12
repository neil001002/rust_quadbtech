//Reverse a string in Rust


fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    let s = "hello";
    let reversed = reverse_string(s);
    println!("Original: {}", s);
    println!("Reversed: {}", reversed);
}
