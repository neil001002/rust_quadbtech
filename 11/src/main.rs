// Merge two sorted arrays in Rust

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    // Append remaining elements from arr1
    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }

    // Append remaining elements from arr2
    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    result
}

fn main() {
    let arr1 = [1, 3, 5, 7];
    let arr2 = [2, 4, 6, 8];
    let merged = merge_sorted_arrays(&arr1, &arr2);

    println!("Merged sorted array: {:?}", merged);
}
