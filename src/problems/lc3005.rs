//! ## Leetcode 3005. Count Elements With Maximum Frequency
//! https://leetcode.com/problems/count-elements-with-maximum-frequency
//! - `Easy`; `Independently Solved`; `2024-03-08`;
//!
//! Since the range of `num` in `nums` is super narrow, I can use a fix-sized array to count the frequency of numbers.

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut counts: [usize; 100] = [0; 100];
    for num in nums {
        counts[num as usize - 1] += 1;
    }
    let mut curr_max = 0;
    let mut curr_count = 0;
    for count in counts {
        if count > curr_max {
            curr_max = count;
            curr_count = 1;
        } else if count == curr_max {
            curr_count += 1;
        }
    }
    (curr_max * curr_count) as i32
}
