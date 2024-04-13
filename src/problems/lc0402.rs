//! ## Leetcode 402. Remove K Digits
//! https://leetcode.com/problems/remove-k-digits
//! - `Medium`; `Learned from Solution`; `2024-04-13`;
//!
//! I think the intuition would be we want the smallest digits to be at the front.

pub fn remove_kdigits(num: String, mut k: i32) -> String {
    let len = num.len();
    if len == 0 {
        return "0".to_string();
    }

    let mut stk: Vec<char> = Vec::with_capacity(len);

    for curr in num.chars() {
        while k > 0 && !stk.is_empty() && stk.last().expect("Checked len") > &curr {
            stk.pop();
            k -= 1;
        }
        stk.push(curr);
    }
    while k > 0 {
        stk.pop();
        k -= 1;
    }
    let mut ans: String = String::with_capacity(stk.len());
    for c in stk {
        if c == '0' && ans.is_empty() {
            continue;
        }
        ans.push(c);
    }
    if ans.is_empty() {
        "0".to_string()
    } else {
        ans
    }
}
