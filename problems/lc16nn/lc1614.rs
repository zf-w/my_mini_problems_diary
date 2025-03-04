//! ## Leetcode 1614. Maximum Nesting Depth of the Parentheses
//! https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses
//! - `Easy`; `Independently Solved`; `2024-04-03`;
//!
//! Increase depth when meeting a left bracket and decrease depth when meeting a right bracket.

pub fn max_depth(s: String) -> i32 {
    let mut curr_depth: i32 = 0;
    let mut ans: i32 = 0;
    for c in s.chars() {
        if c == '(' {
            curr_depth += 1;
            ans = ans.max(curr_depth);
        } else if c == ')' {
            curr_depth -= 1;
        }
    }
    ans
}
