//! ## Leetcode 1636. Sort Array by Increasing Frequency
//! https://leetcode.com/problems/sort-array-by-increasing-frequency/
//! - `Easy`; `Independently Solved`; `2024-07-23`;

pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let mut counts: [usize; 201] = [0; 201];
    for num in nums.iter() {
        counts[(*num + 100) as usize] += 1;
    }
    let mut unique_num_count = 0;
    for count in counts.iter() {
        if *count > 0 {
            unique_num_count += 1;
        }
    }
    let mut unique_nums: Vec<(usize, i32)> = Vec::with_capacity(unique_num_count);
    for (i, count) in counts.iter().cloned().enumerate() {
        if count > 0 {
            unique_nums.push((count, -(i as i32 - 100)));
        }
    }
    unique_nums.sort_unstable();
    nums.clear();
    for (count, unique_num) in unique_nums.iter().cloned() {
        for _ in 0..count {
            nums.push(-unique_num)
        }
    }
    nums
}
