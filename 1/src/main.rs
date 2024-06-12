// Implement a function that checks whether a given string is a palindrome or not.

use regex::Regex;

fn is_palindrome(s: &str) -> bool {
    // Create a regex to match non-alphanumeric characters
    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
    
    // Remove non-alphanumeric characters and convert to lowercase
    let cleaned: String = re.replace_all(s, "").to_lowercase();

    // Check if the cleaned string is equal to its reverse
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn main() {
    // Example usage
    println!("{}", is_palindrome("A man, a plan, a canal, Panama")); // True
    println!("{}", is_palindrome("race a car")); // False
    println!("{}", is_palindrome("No 'x' in Nixon")); // True
}
