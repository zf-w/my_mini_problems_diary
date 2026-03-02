//! # Leetcode 3713. Longest Balanced Substring I
//! https://leetcode.com/problems/longest-balanced-substring-i/
//! - y2026m02d12; Hinted;
//! 
//! Super slow haha.

pub fn longest_balanced(s: String) -> i32 {

    let s_byte_arr_ref = s.as_bytes();
    let full_len = s.len();

    if full_len == 1 {
        return 1;
    }

    for len in (3..=full_len).rev() {
        let last_begin_i = full_len - len;

        for begin_i in 0..=last_begin_i {
            let mut count_arr: [usize; 26] = [0; 26];

            // println!("{} {}", begin_i, begin_i + len);

            for byte_i in begin_i..(begin_i + len) {
                count_arr[(s_byte_arr_ref[byte_i] - b'a') as usize] += 1;
            }

            let mut count_iter = count_arr.into_iter();
            let mut first_count = 0;
            
            while let Some(count) = count_iter.next() {
                if count > 0 {
                    first_count = count;
                    break;
                }
            }

            let mut valid_flag = true;
            for count in count_iter {
                // println!("{}", count);
                if count > 0 && count != first_count {
                    valid_flag = false;
                    break;
                }
            }

            if valid_flag {
                return len as i32;
            }
        }
    }

    2
}