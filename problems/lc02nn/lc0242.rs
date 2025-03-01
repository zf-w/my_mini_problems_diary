//! ## Leetcode 242. Valid Anagram
//! https://leetcode.com/problems/valid-anagram
//! - `Easy`; `Independently Solved`; `2023-12-15`;
//! 
//! I'm practicing using "get_mut" with "expect" instead of direct subscriptions.

pub fn is_anagram(s: String, t: String) -> bool {
    let mut count: [i32; 32] = [0; 32];
    let base: usize = 'a' as usize;
    let exp: &str = "expecting a lowercase letter";
    for c in s.chars() {
        *count.get_mut(c as usize - base).expect(exp) += 1;
    }        
    for c in t.chars() {
        *count.get_mut(c as usize - base).expect(exp) -= 1;
    }
    let mut ans = true;
    for i in count.iter() {
        ans &= *i == 0;
    }
    ans
}