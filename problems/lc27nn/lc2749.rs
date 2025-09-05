//! # Leetcode 2749. Minimum Operations to Make the Intger Zero
//! https://leetcode.com/problems/minimum-operations-to-make-the-integer-zero/
//! - `Medium`; `y2025m09d05`; `Learned from Solution`; `0ms`; `2.1mb`; `1 attempt`;
//! Topics: uncategorized.
//! https://leetcode.com/problems/minimum-operations-to-make-the-integer-zero/editorial/?envType=daily-question&envId=2025-09-05&difficulty=EASY&page=1

pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
    let num1: i64 = num1 as i64;
    let num2: i64 = num2 as i64;

    let mut op_num: i64 = 1;

    loop {
        let target = num1 - num2 * op_num;

        if op_num > target {
            return -1;
        }

        if (target.count_ones() as i64) <= op_num {
            return op_num as i32;
        }

        op_num += 1;
    }
}
