//! # Leetcode 2657. Find the Prefix Common Array of Two Arrays
//! https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays/
//! - `Medium`; `y2025m01d14`; `Independently Solved`; `1ms`; `2.3mb`; `1 attempt`;

pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut ans_vec: Vec<i32> = Vec::with_capacity(a.len());
    let mut flags = [false; 50];
    let mut count = 0;
    for (a_num, b_num) in a.into_iter().zip(b) {
        let a_idx = (a_num - 1) as usize;
        let b_idx = (b_num - 1) as usize;
        if flags[a_idx] {
            count += 1;
        } else {
            flags[a_idx] = true;
        }

        if flags[b_idx] {
            count += 1;
        } else {
            flags[b_idx] = true;
        }
        ans_vec.push(count);
    }
    ans_vec
}
