//! ## Leetcode 3100. Water Bottles II
//! https://leetcode.com/problems/water-bottles-ii
//! - `Medium`; `Independently Solved`; `2024-03-30`;
//!
//! I think the simulation approach would work, considering the problem size. I guess there are also quicker mathematical approaches.

pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
    let mut ans = 0;
    while num_bottles >= num_exchange {
        num_bottles = num_bottles - num_exchange + 1;

        ans += num_exchange;
        num_exchange += 1;
        println!("{} {} {}", num_bottles, num_exchange, ans);
    }
    ans + num_bottles
}

#[test]
fn check_case_0() {
    assert_eq!(15, max_bottles_drunk(13, 6))
}
