//! ## Leetcode 670. Maximum Swap
//! https://leetcode.com/problems/maximum-swap/
//! - `Medium`; `Independently Solved`; `y2024m10d16`;

fn helper(num: i32, first_digit_base: i32) -> i32 {
    if num == 0 || first_digit_base == 0 {
        return num;
    }
    let first_digit = num / first_digit_base;
    let mut num_copy = num;
    let mut curr_digit_base: i32 = 1;
    let mut ans_num = num;
    let first_base = first_digit_base * first_digit;
    while num_copy > 0 {
        let curr_digit = num_copy % 10;
        ans_num = ans_num.max(
            num - first_base + curr_digit * first_digit_base - curr_digit * curr_digit_base
                + first_digit * curr_digit_base,
        );
        num_copy /= 10;
        curr_digit_base *= 10;
    }
    ans_num.max(first_base + helper(num - first_base, first_digit_base / 10))
}

pub fn maximum_swap(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    let mut first_digit_base: i32 = 1;
    let mut num_copy = num;
    while num_copy > 0 {
        num_copy /= 10;
        first_digit_base *= 10;
    }
    first_digit_base /= 10;

    helper(num, first_digit_base)
}
