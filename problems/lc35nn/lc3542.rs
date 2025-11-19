//! # Leetcode 3542. Minimum Operations to Convert All Elements to Zero
//! https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero/
//! - `Medium`; `y2025m11d10`; `Learned from Editorial`; `23ms`; `3.4mb`; `2 attempts`;
//! https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero/editorial
//! Topics: stack.

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut stk = Vec::with_capacity(nums.len());

    let mut ans_op_num = 0;

    for num in nums {
        while stk.is_empty() == false && (stk.last().cloned().expect("len > 0") > num) {
            stk.pop();
        }

        if num == 0 {
            continue;
        }

        if stk.is_empty() == true || (stk.last().cloned().expect("len > 0") < num) {
            ans_op_num += 1;
            stk.push(num);
        }
    }

    ans_op_num
}