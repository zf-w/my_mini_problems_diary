//! # Leetcode 3634. Minimum Removals to Balance Array
//! https://leetcode.com/problems/minimum-removals-to-balance-array/
//! y2026m02d06; Independently Solved;

pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();

    let mut end_iter = nums.iter().cloned();
    let mut begin_iter = nums.iter().cloned();

    let mut min_val = 0;

    let mut ok_len = 0;
    let mut ans_max_len = 0;

    for num in end_iter {
        ok_len += 1;

        while ((min_val as i64 * k as i64)) < (num as i64) { // Be careful with overflowing...
            min_val = begin_iter.next().expect("begin <= end");
            ok_len -= 1;
        }
        // println!("{} {}", min_val, ok_len);
        ans_max_len = ans_max_len.max(ok_len);
    }

    (nums.len() as i32) - ans_max_len - 1
}