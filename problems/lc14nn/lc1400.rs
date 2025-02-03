//! # Leetcode 1400. Construct K Palindrome Strings
//! https://leetcode.com/problems/construct-k-palindrome-strings/
//! - `Medium`; `y2025m01d11`; `Independently Solved`; `0ms`; `2.5mb`; `1 attempt`;

pub fn can_construct(s: String, mut k: i32) -> bool {
    const CHAR_LOWER_A_U8: u8 = b'a';
    #[inline]
    fn c_u8_to_idx(c_u8: u8) -> usize {
        (c_u8 - CHAR_LOWER_A_U8) as usize
    }

    let mut c_count_arr: [usize; 26] = [0; 26];
    let mut total_char_count: i32 = 0;

    for c_u8 in s.as_bytes().iter().cloned() {
        c_count_arr[c_u8_to_idx(c_u8)] += 1;
        total_char_count += 1;
    }

    let mut odd_count: i32 = 0;

    for count in c_count_arr.iter() {
        if count % 2 == 1 {
            odd_count += 1;
        }
    }

    let used_count = odd_count.min(k - 1);

    odd_count -= used_count;
    k -= used_count;
    total_char_count -= used_count;

    if odd_count > 1 {
        return false;
    }

    k <= total_char_count // Found forgetting equal sign here. Was `k < total_char_count` before.
}
