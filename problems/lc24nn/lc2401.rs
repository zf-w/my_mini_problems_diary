//! # Leetcode 2401. Longest Nice Subarray
//! https://leetcode.com/problems/longest-nice-subarray/
//! - `Medium`; `y2025m03d17`; `Independently Solved`; `96ms`; `4mb`; `1 attempt`;
//! Topics" sliding_window.
//!
//! Yeah! Truly zero debugging and one attempt pass!
//!

pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut bit_count_arr: [usize; 32] = [0; 32];

    fn push_num(bit_count_arr_mut_ref: &mut [usize; 32], mut num: i32, prev_num_mut_ref: &mut i32) {
        let mut bit_idx: usize = 0;
        while num > 0 {
            if num & 1 == 1 {
                let bit_arr_entry_mut_ref = &mut bit_count_arr_mut_ref[bit_idx];
                if *bit_arr_entry_mut_ref == 0 {
                    *prev_num_mut_ref |= 1 << bit_idx;
                }
                *bit_arr_entry_mut_ref += 1;
            }
            bit_idx += 1;
            num >>= 1;
        }
    }
    fn pop_num(bit_count_arr_mut_ref: &mut [usize; 32], mut num: i32, prev_num_mut_ref: &mut i32) {
        let mut bit_idx: usize = 0;
        while num > 0 {
            if num & 1 == 1 {
                let bit_arr_entry_mut_ref = &mut bit_count_arr_mut_ref[bit_idx];
                *bit_arr_entry_mut_ref -= 1;
                if *bit_arr_entry_mut_ref == 0 {
                    *prev_num_mut_ref &= !(1 << bit_idx);
                }
            }
            bit_idx += 1;
            num >>= 1;
        }
    }
    let mut begin_iter = nums.iter().cloned();
    let end_iter = nums.iter().cloned();

    let mut prev_bits_num = 0;
    let mut curr_len: usize = 0;

    let mut ans_len: usize = 0;
    for end_num in end_iter {
        while end_num & prev_bits_num > 0 {
            let begin_num = begin_iter.next().expect("begin < end");
            pop_num(&mut bit_count_arr, begin_num, &mut prev_bits_num);
            curr_len -= 1;
        }

        push_num(&mut bit_count_arr, end_num, &mut prev_bits_num);
        curr_len += 1;
        ans_len = ans_len.max(curr_len);
    }

    ans_len as i32
}
