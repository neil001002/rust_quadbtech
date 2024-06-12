// Given a sorted array of integers, implement a function that returns the median of the array.

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len == 0 {
        panic!("Cannot compute the median of an empty array");
    }
    
    if len % 2 == 0 {
        let mid1 = len / 2 - 1;
        let mid2 = len / 2;
        (arr[mid1] as f64 + arr[mid2] as f64) / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5, 6];
    
    println!("The median of arr1 is: {}", median(&arr1)); // Should print 3.0
    println!("The median of arr2 is: {}", median(&arr2)); // Should print 3.5
}
