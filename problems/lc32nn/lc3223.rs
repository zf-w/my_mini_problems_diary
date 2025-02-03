//! # Leetcode 3223. Minimum Length of String After Operations
//! https://leetcode.com/problems/minimum-length-of-string-after-operations/
//! - `Medium`; `y2025m01d12`; `Independently Solved`; `3ms`; `2.8mb`; `1 attempt`;

pub fn minimum_length(s: String) -> i32 {
    let mut c_count: [usize; 26] = [0; 26];
    const CHAR_LOWER_A_U8: u8 = b'a';
    #[inline]
    fn calc_c_idx(c_u8: u8) -> usize {
        (c_u8 - CHAR_LOWER_A_U8) as usize
    }

    for c_u8 in s.bytes() {
        c_count[calc_c_idx(c_u8)] += 1;
    }

    let mut ans_len = 0;
    for count in c_count {
        if count % 2 == 1 {
            ans_len += 1;
        } else if count != 0 {
            ans_len += 2;
        }
    }
    ans_len
}
