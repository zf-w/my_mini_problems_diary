//! Leetcode 930. Binary Subarrays With Sum
//! https://leetcode.com/problems/binary-subarrays-with-sum
//! - `Medium`; `Independently Solved`; `2024-03-14`;
//!
//! I guess, for loops under this or similar situations, four cases need to be considered: the general case, the begin case, the end case, and whether begin equals end.

pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let len = nums.len();
    let mut pos_1 = Vec::with_capacity(len);
    for (i, num) in nums.iter().enumerate() {
        if *num == 1 {
            pos_1.push(i);
        }
    }
    let mut begin: usize = 0;
    let mut end: usize = begin + goal as usize;
    let pos_1_len = pos_1.len();
    let mut ans: i32 = 0;
    if goal > 0 {
        while end <= pos_1_len {
            let i0 = if begin == 0 { 0 } else { pos_1[begin - 1] + 1 };
            let i1 = pos_1[begin];
            let i2 = pos_1[end - 1];
            let i3 = if end == pos_1_len {
                len - 1
            } else {
                pos_1[end] - 1
            };
            // println!("{} {} {} {}", i0,i1,i2,i3);
            ans += ((1 + i1 - i0) * (1 + i3 - i2)) as i32;
            end += 1;
            begin += 1;
        }
    } else {
        fn cal_len(len: usize) -> i32 {
            (((1 + len) * len) / 2) as i32
        }
        if pos_1_len > 0 {
            end = 0;
            ans += cal_len(pos_1[end]);
            end = 1;
            while end < pos_1_len {
                ans += cal_len(pos_1[end] - pos_1[end - 1] - 1);
                end += 1;
            }
            ans += cal_len(len - pos_1[end - 1] - 1);
        } else {
            ans += cal_len(len);
        }
    }
    ans
}
