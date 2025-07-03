//! # Leetcode 2566. Maximum Difference by Remapping a Digit
//! https://leetcode.com/problems/maximum-difference-by-remapping-a-digit/
//! `Easy`; `y2025m06d14`; `Independently Solved`; `0ms`; `2.1mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn min_max_difference(num: i32) -> i32 {
    fn calc_max_num(mut num: i32) -> i32 {
        let mut div: i32 = 100000000;
        let mut swap_info_opt: Option<(i32, i32)> = None;
        let mut ans = 0;

        for i in 0..8 {
            let digit = num / div;
            num = num % div;

            ans = ans * 10
                + if let Some((from_digit, to_digit)) = swap_info_opt {
                    if digit == from_digit {
                        to_digit
                    } else {
                        digit
                    }
                } else {
                    if digit == 9 || digit == 0 {
                        digit
                    } else {
                        swap_info_opt = Some((digit, 9));
                        9
                    }
                };

            div /= 10;
        }
        ans
    }

    fn calc_min_num(mut num: i32) -> i32 {
        let mut div: i32 = 100000000;
        let mut swap_info_opt: Option<(i32, i32)> = None;
        let mut ans = 0;

        for i in 0..8 {
            let digit = num / div;
            num = num % div;

            ans = ans * 10
                + if let Some((from_digit, to_digit)) = swap_info_opt {
                    if digit == from_digit {
                        to_digit
                    } else {
                        digit
                    }
                } else {
                    if digit == 0 {
                        digit
                    } else {
                        swap_info_opt = Some((digit, 0));
                        0
                    }
                };

            div /= 10;
        }
        ans
    }

    calc_max_num(num) - calc_min_num(num)
}
