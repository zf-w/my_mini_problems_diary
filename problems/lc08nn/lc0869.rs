//! # Leetcode 869. Reordered Power of 2
//! https://leetcode.com/problems/reordered-power-of-2/
//! - `Medium`; `y2025m08d10`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: count_tracking;

pub fn reordered_power_of2(n: i32) -> bool {
    const DIGIT_NUM: usize = 10;
    const COUNT_ARR_LEN: usize = DIGIT_NUM + 1;
    let mut digit_count_arr: [usize; COUNT_ARR_LEN] = [0; COUNT_ARR_LEN];

    fn add_digit_to_digit_count_arr(
        digit_count_arr_mut_ref: &mut [usize; COUNT_ARR_LEN],
        mut num: i32,
    ) {
        while num > 0 {
            let digit = (num % 10) as usize;

            let entry_mut_ref = &mut digit_count_arr_mut_ref[digit];
            *entry_mut_ref += 1;
            if *entry_mut_ref == 1 {
                digit_count_arr_mut_ref[DIGIT_NUM] += 1;
            }

            num /= 10;
        }
    }

    fn check_digit_count(
        digit_count_arr_ref: &[usize; COUNT_ARR_LEN],
        mut to_check_num: i32,
    ) -> bool {
        let mut digit_count_arr: [usize; COUNT_ARR_LEN] = *digit_count_arr_ref;

        while to_check_num > 0 {
            let digit = (to_check_num % 10) as usize;

            let entry_mut_ref = &mut digit_count_arr[digit];

            if *entry_mut_ref == 0 {
                return false;
            }

            *entry_mut_ref -= 1;

            if *entry_mut_ref == 0 {
                digit_count_arr[DIGIT_NUM] -= 1;
            }

            to_check_num /= 10;
        }

        digit_count_arr[DIGIT_NUM] == 0
    }

    add_digit_to_digit_count_arr(&mut digit_count_arr, n);

    let mut pow_2_num = 1;
    for _ in 0..31 {
        if check_digit_count(&digit_count_arr, pow_2_num) == true {
            return true;
        }

        pow_2_num <<= 1;
    }

    false
}