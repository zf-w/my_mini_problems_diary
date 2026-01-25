//! # Leetcode 1984. Minimum Difference Between Highest and Lowest of K Scores
//! https://leetcode.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/
//! y2026m01d25; Independently Solved;

pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut begin_iter = nums.iter().cloned();
    let mut last_iter = nums.iter().cloned();

    for i in 1..k {
        last_iter.next();
    }

    let mut ans_min = i32::MAX;

    for (begin_num, last_num) in begin_iter.zip(last_iter) {
        ans_min = ans_min.min(last_num - begin_num);
    }

    ans_min
}