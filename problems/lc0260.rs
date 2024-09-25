//! ## Leetcode 260. Single Number III
//! https://leetcode.com/problems/single-number-iii/
//! - `Medium`; `Learned from Solution`; `2024-05-30`;
//!
//! I have read solution: https://leetcode.com/problems/single-number-iii/solutions/5233120/find-significant-bit-of-xor-split-in-2-group-with-math-proof-0ms-beats-100/
//!
//! I know the XOR of all numbers gives me the XOR of the two single numbers, but I didn't think of dividing the numbers into two groups based on their different digits. That idea is brilliant.
//!

pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    fn get_mask_for_a_bit_one(n: i32) -> i32 {
        for i in 0..32 {
            if (n >> i) & 1 == 1 {
                return 1 << i;
            }
        }
        unreachable!()
    }
    let mut xor_0 = 0;
    for num_ref in nums.iter() {
        xor_0 ^= *num_ref;
    }
    let diff_mask = get_mask_for_a_bit_one(xor_0);
    xor_0 = 0;
    let mut xor_1 = 0;
    for num_ref in nums.iter() {
        if *num_ref & diff_mask == 0 {
            xor_0 ^= *num_ref;
        } else {
            xor_1 ^= *num_ref;
        }
    }
    vec![xor_0, xor_1]
}
