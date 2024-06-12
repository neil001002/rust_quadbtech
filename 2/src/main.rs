//Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() as isize - 1;
    let mut result = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid as usize] == target {
            result = Some(mid as usize);
            high = mid - 1; // Move to the left part to find the first occurrence
        } else if arr[mid as usize] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    let arr = [1, 2, 2, 2, 3, 4, 5];
    let target = 2;
    match find_first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}.", target, index),
        None => println!("{} is not in the array.", target),
    }
}
