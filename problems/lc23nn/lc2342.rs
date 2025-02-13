//! # Leetcode 2342. Max Sum of a Pair With Equal Sum of Digits
//! https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/
//! - `Medium`; `y2025m02d12`; `Independently Solved`; `4ms`; `4mb`; `1 attempt`;
//!
//! Note: busy, first copied the official C++ solution: https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/solutions/6389851/max-sum-of-a-pair-with-equal-sum-of-digits
//! Note: But I didn't read it. I was writing my original solution.

pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    use std::collections::{hash_map, HashMap};
    let mut map: HashMap<i32, (bool, i32, i32)> = HashMap::new();

    fn calc_digit_sum(mut num: i32) -> i32 {
        let mut ans_sum = 0;
        while (num > 0) {
            ans_sum += num % 10;
            num /= 10;
        }
        ans_sum
    }

    fn update_val(num: i32, val_mut_ref: &mut (bool, i32, i32)) {
        let (is_full_flag_mut_ref, num_0_mut_ref, num_1_mut_ref) = val_mut_ref;
        if *is_full_flag_mut_ref == false {
            if num < *num_0_mut_ref {
                *num_1_mut_ref = num;
            } else {
                *num_1_mut_ref = *num_0_mut_ref;
                *num_0_mut_ref = num;
            }
            *is_full_flag_mut_ref = true;
        } else {
            if (num > *num_0_mut_ref) {
                *num_1_mut_ref = *num_0_mut_ref;
                *num_0_mut_ref = num;
            } else if (num > *num_1_mut_ref) {
                *num_1_mut_ref = num;
            }
        }
    }

    let mut ans_max = -1;
    for num in nums {
        let key = calc_digit_sum(num);
        match map.entry(key) {
            hash_map::Entry::Occupied(mut occupied_entry) => {
                let val_mut_ref = occupied_entry.get_mut();
                update_val(num, val_mut_ref);
                if (val_mut_ref.0 == true) {
                    ans_max = ans_max.max(val_mut_ref.1 + val_mut_ref.2);
                }
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert((false, num, 0));
            }
        }
    }
    ans_max
}
