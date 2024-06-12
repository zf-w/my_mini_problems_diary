//! ## Leetcode 1122. Relative Sort Array
//! https://leetcode.com/problems/relative-sort-array/
//! - `Easy`; `Independently Solved`; `2024-06-11`;
//!
//! We can count the number of occurrences of each element in the second array to quickly build the first part of the answer due to a narrow range of elements. We can sort the second part of the elements not existing in `arr2` at the end.

pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut set_arr: [usize; 1001] = [0; 1001];
    for num in arr2.iter().cloned() {
        set_arr[num as usize] = 1;
    }
    let mut arr2_part_len: usize = 0;
    for num in arr1.iter().cloned() {
        if set_arr[num as usize] > 0 {
            set_arr[num as usize] += 1;
            arr2_part_len += 1;
        }
    }
    let mut ans_arr = Vec::with_capacity(arr1.len());

    for curr in arr2.iter().cloned() {
        let count = set_arr[curr as usize] - 1;
        for _ in 1..count {
            ans_arr.push(curr);
        }
    }
    for num in arr1.iter().cloned() {
        if set_arr[num as usize] == 0 {
            ans_arr.push(num);
        }
    }
    ans_arr[arr2_part_len..].sort();

    ans_arr
}
