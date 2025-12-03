//! # Leetcode 2011. Final Value of Variable After Performing Operations
//! https://leetcode.com/problems/final-value-of-variable-after-performing-operations/
//! - `Easy`; `y2025m10d19`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut ans_val = 0;
    for op in operations {
        if op.as_bytes()[1] == b'+' {
            ans_val += 1;
        } else {
            ans_val -= 1;
        }
    }
    ans_val
}