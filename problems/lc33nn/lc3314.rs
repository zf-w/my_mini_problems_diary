//! # Leetcode 3314. Construct the Minimum Bitwise Array I
//! https://leetcode.com/problems/construct-the-minimum-bitwise-array-i/
//! y2026m01d20; Independently Solved

pub fn min_bitwise_array(mut nums: Vec<i32>) -> Vec<i32> {
    fn calc_bitwise_num(num: i32) -> i32 {
        if num & 1 == 0 {
            return -1;
        }

        let mut idx_bitmask = 1;
        while (num & idx_bitmask) > 0 {
            idx_bitmask <<= 1;
        }

        num ^ (idx_bitmask >> 1)
    }

    for num_mut_ref in nums.iter_mut() {
        *num_mut_ref = calc_bitwise_num(*num_mut_ref);
    }

    nums
}