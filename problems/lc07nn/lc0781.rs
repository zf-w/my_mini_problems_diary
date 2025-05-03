//! # Leetcode 781. Rabbits in Forest
//! https://leetcode.com/problems/rabbits-in-forest/
//! - `Medium`; `y2025m04d20`; `Independently Solved`; `0ms`; `2.1mb`; `2 attempts`;
//! Topics: uncategorized.

pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut count_arr: [usize; 1001] = [0; 1001]; // Was 1000 before haha.
    let mut ans_num = 0;
    for answer in answers {
        let group_size = (answer + 1) as usize;
        let count_entry = &mut count_arr[group_size];
        if *count_entry % group_size == 0 {
            ans_num += group_size;
        }
        *count_entry += 1;
    }
    ans_num as i32
}
