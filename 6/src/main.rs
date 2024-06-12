// Implement a function that finds the longest common prefix of a given set of strings.

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let mut prefix = strings[0].clone();
    
    for s in &strings[1..] {
        let mut new_prefix = String::new();
        for (c1, c2) in prefix.chars().zip(s.chars()) {
            if c1 != c2 {
                break;
            }
            new_prefix.push(c1);
        }
        prefix = new_prefix;
        if prefix.is_empty() {
            break;
        }
    }
    
    prefix
}

fn main() {
    let strings1 = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let strings2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

    println!("Longest common prefix of {:?}: {}", strings1, longest_common_prefix(&strings1));
    println!("Longest common prefix of {:?}: {}", strings2, longest_common_prefix(&strings2));
}
