//! ## Leetcode 791. Custom Sort String
//! https://leetcode.com/problems/custom-sort-string
//! - `Medium`; `Independently Solved`; `2024-03-10`;
//!
//! Need to be careful about those characters not in the order string and don't forget to include 'z' in the loop.

pub fn custom_sort_string(order: String, s: String) -> String {
    let mut counts: [usize; 26] = [0; 26];
    let base_a = 'a' as usize;
    for c in s.as_bytes() {
        counts[*c as usize - base_a] += 1;
    }
    let mut ans: String = String::with_capacity(s.len());
    for curr in order.as_bytes() {
        let count = counts[*curr as usize - base_a];
        counts[*curr as usize - base_a] = 0;
        for _ in 0..count {
            ans.push(*curr as char);
        }
    }
    for curr in ('a' as usize)..('z' as usize) {
        let count = counts[curr as usize - base_a];
        for _ in 0..count {
            ans.push((curr as u8) as char);
        }
    }
    ans
}
