//! # Leetcode 2411. Smallest Subarrays With Maximum Bitwise OR
//! https://leetcode.com/problems/smallest-subarrays-with-maximum-bitwise-or/
//! - `Medium`; `y2025m07d29`; `Independently Solved`; `25ms`; `3.6mb`; `5 attempts`;
//! Topics: sliding_window.

pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
    const I32_BIT_NUM: usize = 32;
    let mut suffix_bit_sum: [usize; I32_BIT_NUM + 1] = [0; I32_BIT_NUM + 1];

    let mut prefix_bit_sum: [usize; I32_BIT_NUM + 1] = [0; I32_BIT_NUM + 1];

    fn push_to_bit_sum(bit_sum_mut_ref: &mut [usize; I32_BIT_NUM + 1], mut push_num: i32) {
        let mut bit_i = 0;

        while push_num > 0 {
            if push_num & 1 == 1 {
                let entry_mut_ref = &mut bit_sum_mut_ref[bit_i];
                *entry_mut_ref += 1;
                if *entry_mut_ref == 1 {
                    bit_sum_mut_ref[I32_BIT_NUM] += 1;
                }
            }
            bit_i += 1;
            push_num >>= 1;
        }
    }

    fn pop_from_bit_sum(bit_sum_mut_ref: &mut [usize; I32_BIT_NUM + 1], mut pop_num: i32) {
        let mut bit_i = 0;

        while pop_num > 0 {
            if pop_num & 1 == 1 {
                let entry_mut_ref = &mut bit_sum_mut_ref[bit_i];
                *entry_mut_ref -= 1;
                if *entry_mut_ref == 0 {
                    bit_sum_mut_ref[I32_BIT_NUM] -= 1;
                }
            }
            bit_i += 1;
            pop_num >>= 1;
        }
    }

    for push_num in nums
        .iter()
        .cloned() {
            push_to_bit_sum(&mut suffix_bit_sum, push_num);
    }

    let mut end_i: usize = 0;
    let mut end_iter = nums.iter().cloned().enumerate();

    let mut ans_vec: Vec<i32> = Vec::with_capacity(nums.len());

    for (begin_i, pop_num) in nums.iter().cloned().enumerate() {
        while prefix_bit_sum[I32_BIT_NUM] < suffix_bit_sum[I32_BIT_NUM] {
            if let Some((end_i_update, push_num)) = end_iter.next() {
                end_i = end_i_update;
                push_to_bit_sum(&mut prefix_bit_sum, push_num);
            } else {
                break;
            }
        }
        let ans = ((end_i - begin_i + 1) as i32).max(1);
        ans_vec.push(ans);
        pop_from_bit_sum(&mut prefix_bit_sum, pop_num);
        pop_from_bit_sum(&mut suffix_bit_sum, pop_num);

        // println!("{} {}", prefix_bit_sum[I32_BIT_NUM], suffix_bit_sum[I32_BIT_NUM]);
    }

    ans_vec
}