//! # Leetcode 2226. Maximum Candies Allocated to K Children
//! https://leetcode.com/problems/maximum-candies-allocated-to-k-children/
//! - `Medium`; `y2025m03d14`; `Independently Solved`; `7ms`; `3.5mb`; `2 attempt`;
//! Topics: binary_search.

pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let candy_sum: i64 = candies.iter().map(|v| -> i64 { (*v) as i64 }).sum(); // Forgot to take sum in `i64`` instead of `i32``.
    let mut begin_num: i64 = 0;
    let mut end_num: i64 = ((candy_sum as i64) / k) + 1; // Leading to a wrong end boundary.

    let check_closure = |mid_num: i64| -> bool {
        let mut child_satisfied_num: i64 = 0;
        for candy_pile in candies.iter().cloned() {
            child_satisfied_num += candy_pile as i64 / mid_num;
            if child_satisfied_num >= k {
                return true;
            }
        }
        false
    };

    // println!("{} {}", check_closure(1000000), end_num);
    while begin_num < end_num {
        let mid_num = (begin_num + end_num) / 2;
        if mid_num == 0 || check_closure(mid_num) == true {
            begin_num = mid_num + 1;
        } else {
            end_num = mid_num;
        }
    }
    begin_num as i32 - 1
}
