//! ## Leetcode 2028. Find Missing Observations
//! https://leetcode.com/problems/find-missing-observations/
//! - `Medium`; `Independently Solved`; `2024-09-04`;

pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let missing_len = n as usize;
    let m = rolls.len();
    let m_sum: i32 = rolls.iter().sum();
    let total_sum = (missing_len + m) as i32 * mean;
    let mut need_sum = total_sum - m_sum;

    let mut ans_vec: Vec<i32> = Vec::with_capacity(missing_len);
    for i in 0..missing_len {
        let to_push = need_sum / ((missing_len - i) as i32);
        ans_vec.push(to_push);
        need_sum -= to_push;
    }
    ans_vec
}
