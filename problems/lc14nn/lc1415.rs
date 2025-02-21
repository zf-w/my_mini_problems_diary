//! # Leetcode 1415. The k-th Lexicographical String of All Happy Strings of Length n
//! https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/
//! - `Medium`; `y2025m02d19`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;
//!
//! Topic: recursion.

pub fn get_happy_string(n: i32, k: i32) -> String {
    let n = n as usize;
    let k = k as usize - 1;
    let mut ans_string: String = String::with_capacity(n);

    fn get_group_size(len: usize) -> usize {
        1 << len
    }

    const LETTER_0: char = 'a';
    const LETTER_1: char = 'b';
    const LETTER_2: char = 'c';

    let mut prev_char: char;
    let mut curr_group_idx = 0;
    let mut curr_remaining_len = n - 1;

    let mut curr_group_size = get_group_size(curr_remaining_len);

    if (0..curr_group_size).contains(&k) {
        ans_string.push(LETTER_0);
        prev_char = LETTER_0;
    } else if (curr_group_size..(2 * curr_group_size)).contains(&k) {
        ans_string.push(LETTER_1);
        prev_char = LETTER_1;
        curr_group_idx = curr_group_size;
    } else if ((2 * curr_group_size)..(3 * curr_group_size)).contains(&k) {
        ans_string.push(LETTER_2);
        prev_char = LETTER_2;
        curr_group_idx = 2 * curr_group_size;
    } else {
        return "".to_string();
    }

    if curr_remaining_len == 0 {
        // Be careful with this hahaha. Memory Limit Exceeded otherwise.
        return ans_string;
    }

    loop {
        curr_remaining_len -= 1;
        curr_group_size = get_group_size(curr_remaining_len);
        for next_letter in [LETTER_0, LETTER_1, LETTER_2] {
            if next_letter == prev_char {
                continue;
            }

            if k >= curr_group_idx + curr_group_size {
                curr_group_idx += curr_group_size;
                continue;
            }

            ans_string.push(next_letter);
            prev_char = next_letter;
            break;
        }
        if curr_remaining_len == 0 {
            break;
        }
    }

    ans_string
}
