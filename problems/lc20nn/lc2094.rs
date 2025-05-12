//! # Leetcode 2094. Finding 3-Digit Even Numbers
//! https://leetcode.com/problems/finding-3-digit-even-numbers/
//! - `Easy`; `y2025m05d12`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
    let mut digit_ocur_count_arr: [usize; 10] = [0; 10];

    for digit in digits {
        digit_ocur_count_arr[digit as usize] += 1;
    }

    let mut ans_vec: Vec<i32> = Vec::with_capacity(1000);

    for digit_0_i in 1..10usize {
        if digit_ocur_count_arr[digit_0_i] == 0 {
            continue;
        }

        digit_ocur_count_arr[digit_0_i] -= 1;

        for digit_1_i in 0..10usize {
            if digit_ocur_count_arr[digit_1_i] == 0 {
                continue;
            }

            digit_ocur_count_arr[digit_1_i] -= 1;

            for digit_2_half in 0..5usize {
                let digit_2_i = digit_2_half << 1;
                if digit_ocur_count_arr[digit_2_i] == 0 {
                    continue;
                }

                ans_vec.push((digit_0_i * 100 + digit_1_i * 10 + digit_2_i) as i32);
            }

            digit_ocur_count_arr[digit_1_i] += 1;
        }

        digit_ocur_count_arr[digit_0_i] += 1;
    }

    ans_vec
}
