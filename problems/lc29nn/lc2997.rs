//! ## Leetcode 2997. Minimum Number of Operations to Make Array XOR Equal to K
//! https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k
//! - `Medium`; `Independently Solved`; `2024-04-30`;
//!
//! We can check the number of different bits between the result of Array XOR and the target number. Reversing the bits in the corresponding position in any element in the array will reflect on the final result.

pub fn min_operations(nums: Vec<i32>, mut k: i32) -> i32 {
    let mut xor_ans = 0;
    for num in nums {
        xor_ans ^= num;
    }
    let mut ans = 0;
    while k > 0 {
        if xor_ans & 1 != k & 1 {
            ans += 1;
        }
        xor_ans >>= 1;
        k >>= 1;
    }
    ans
}
