//! Leetcode 2864. Maxiumu Odd Binary Number
//! https://leetcode.com/problems/maximum-odd-binary-number
//! - `Easy`; `Independently Solved`; `2024-02-29`;
//!
//! Indeed, String is a complex data structure considering the "linked list" nature of UTF-8 design. I guess treating it as a byte array would be more efficient if we knew the String contained only one-byte characters.

pub fn maximum_odd_binary_number(s: String) -> String {
    let str_vec = s.as_bytes();
    let len = str_vec.len();
    let one = '1' as u8;
    let zero = '0' as u8;
    let mut count: usize = 0;
    for c in str_vec.iter() {
        if *c == one {
            count += 1;
        }
    }

    let mut ans_str_vec = vec![0; len];

    for i in 0..(count - 1) {
        ans_str_vec[i] = one;
    }
    for i in (count - 1)..(len - 1) {
        ans_str_vec[i] = zero;
    }
    ans_str_vec[len - 1] = one;
    String::from_utf8(ans_str_vec).expect("Should be valid")
}
