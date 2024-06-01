//! ## Leetcode 1208. Get Equal Substrings Within Budget
//! https://leetcode.com/problems/get-equal-substrings-within-budget/
//! - `Medium`; `Learned from Solution`; `2024-05-28`;
//!
//! We can use a sliding window to find the maximum valid length of substrings within budget. As I can remember, I was busy working on my project. I scanned the tags and got the idea of the sliding window. But, perhaps the time is short, I chose to view someone's solution, but I forgot which one it was. Might be the one below.
//!
//! https://leetcode.com/problems/get-equal-substrings-within-budget/solutions/5218505/97-44-easy-solution-with-explanation/
//!

pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let len = s.len();
    let mut ans_len: usize = 0;
    let mut cost = 0;
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    let mut begin_i: usize = 0;

    let calc_cost = |i: usize| -> i32 {
        let s_c = s_bytes[i];
        let t_c = t_bytes[i];
        (if s_c > t_c { s_c - t_c } else { t_c - s_c }) as i32
    };

    for i in 1..=len {
        let curr_cost = calc_cost(i - 1);
        cost += curr_cost;
        while cost > max_cost {
            cost -= calc_cost(begin_i);
            begin_i += 1;
        }
        ans_len = ans_len.max(i - begin_i);
    }
    ans_len as i32
}
