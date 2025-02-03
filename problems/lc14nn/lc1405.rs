//! ## Leetcode 1405. Longest Happy String
//! https://leetcode.com/problems/longest-happy-string/
//! - `Medium`; `Learned from Solution`; `y2024m10d16`;
//!
//! I read this: https://leetcode.com/problems/longest-happy-string/solutions/5918628/how-to-easily-solve-the-longest-happy-string-problem-in-multiple-languages
//!
//! But I guess using a priority queue for three elements is not quite necessary. So, I simply used a bunch of `if` statements.
//!
//! At first, I didn't come up with an idea about when to push two characters and when to push one character.

pub fn longest_diverse_string(mut a: i32, mut b: i32, mut c: i32) -> String {
    let mut ans_string = String::new();
    const LOWER_A_CHAR: char = 'a';
    const LOWER_B_CHAR: char = 'b';
    const LOWER_C_CHAR: char = 'c';

    fn push_to_ans(
        c: char,
        num_mut_ref: &mut i32,
        prev_char_info_mut_ref: &mut (char, bool),
        ans_string_mut_ref: &mut String,
    ) -> bool {
        let mut ans_flag = true;
        if *num_mut_ref > 0 {
            ans_string_mut_ref.push(c);
            *num_mut_ref -= 1;
            ans_flag = false;
        }
        if prev_char_info_mut_ref.0 == c {
            *prev_char_info_mut_ref = (c, true);
        } else {
            *prev_char_info_mut_ref = (c, false);
        }

        ans_flag
    }

    let mut prev_char_info: (char, bool) = ('x', false);

    let mut end_flag = if a >= b && a >= c {
        push_to_ans(LOWER_A_CHAR, &mut a, &mut prev_char_info, &mut ans_string)
    } else if b >= a && b >= c {
        push_to_ans(LOWER_B_CHAR, &mut b, &mut prev_char_info, &mut ans_string)
    } else {
        push_to_ans(LOWER_C_CHAR, &mut c, &mut prev_char_info, &mut ans_string)
    };

    while !end_flag {
        if prev_char_info.1 == true {
            if prev_char_info.0 == LOWER_A_CHAR {
                end_flag = if b >= c {
                    push_to_ans(LOWER_B_CHAR, &mut b, &mut prev_char_info, &mut ans_string)
                } else {
                    push_to_ans(LOWER_C_CHAR, &mut c, &mut prev_char_info, &mut ans_string)
                };
            } else if prev_char_info.0 == LOWER_B_CHAR {
                end_flag = if a >= c {
                    push_to_ans(LOWER_A_CHAR, &mut a, &mut prev_char_info, &mut ans_string)
                } else {
                    push_to_ans(LOWER_C_CHAR, &mut c, &mut prev_char_info, &mut ans_string)
                };
            } else {
                end_flag = if a >= b {
                    push_to_ans(LOWER_A_CHAR, &mut a, &mut prev_char_info, &mut ans_string)
                } else {
                    push_to_ans(LOWER_B_CHAR, &mut b, &mut prev_char_info, &mut ans_string)
                };
            }
        } else {
            end_flag = if a >= b && a >= c {
                push_to_ans(LOWER_A_CHAR, &mut a, &mut prev_char_info, &mut ans_string)
            } else if b >= a && b >= c {
                push_to_ans(LOWER_B_CHAR, &mut b, &mut prev_char_info, &mut ans_string)
            } else {
                push_to_ans(LOWER_C_CHAR, &mut c, &mut prev_char_info, &mut ans_string)
            };
        }
    }
    ans_string
}
