//! # Leetcode 2302. Count Subarrays With Score Less Than K
//! https://leetcode.com/problems/count-subarrays-with-score-less-than-k/
//! - `Hard`; `y2025m04d28`; `Independently Solved`; `0ms`; `3.4mb`; `1 attempt`;
//! Topics: sliding_window.

pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
    let mut sum: i64 = 0;
    let mut count: i64 = 0;
    let mut begin_iter = nums.iter().cloned();
    let end_iter = nums.iter().cloned();

    let mut ans_count = 0;

    for add_num in end_iter {
        sum += add_num as i64;
        count += 1;

        while sum * count >= k {
            let pop_num = begin_iter.next().expect("begin < end");
            count -= 1;
            sum -= pop_num as i64;
        }

        ans_count += count;
    }

    ans_count
}
