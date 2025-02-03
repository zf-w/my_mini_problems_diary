//! ## Leetcode 1404. Number of Steps to Reduce a Number in Binary Representation to One
//! https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/
//! - `Medium`; `Independently Solved`; `2024-05-28`
//!
//! When the last digit is zero, we can remove it. When the last digit is one, we can turn all the previous connected ones into zero and the next zero into one.

pub fn num_steps(mut s: String) -> i32 {
    let base_1 = '1' as u8;
    let bytes = unsafe { s.as_bytes_mut() };
    let mut i: usize = 0;
    let len = bytes.len();
    let last = len - 1;
    let mut ans = 0;

    while i < len {
        let curr = bytes[last - i] == base_1;
        if curr {
            ans += 1;
            while i < len && bytes[last - i] == base_1 {
                ans += 1;
                i += 1;
            }
            if i < len {
                bytes[last - i] = base_1;
            }
        } else {
            ans += 1;
            i += 1;
        }
    }
    ans
}
