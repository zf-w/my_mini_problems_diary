//! ## Leetcode 2938. Separate Black and White Balls
//! https://leetcode.com/problems/separate-black-and-white-balls/
//! - `Medium`; `Independently Solved`; `y2024m10d15`;

pub fn minimum_steps(s: String) -> i64 {
    let mut ans_count: usize = 0;
    let mut zero_count: usize = 0;
    for (i, c) in s.char_indices() {
        if c == '0' {
            ans_count += i - zero_count;
            zero_count += 1;
        } else if c == '1' {
            continue;
        } else {
            unreachable!()
        }
    }
    ans_count as i64
}
