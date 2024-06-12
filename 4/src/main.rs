//Implement a function that checks whether a given number is prime or not.

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    // Test the is_prime function with various inputs
    let test_numbers = [0, 1, 2, 3, 4, 5, 16, 17, 18, 19, 20, 23, 24, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101];

    for &num in &test_numbers {
        println!("Is {} a prime number? {}", num, is_prime(num));
    }
}
