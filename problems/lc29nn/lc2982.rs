//! # Leetcode 2982. Find Longest Special Substring That Occurs Thrice II
//! https://leetcode.com/problems/find-longest-special-substring-that-occurs-thrice-ii/
//! - `Medium`; `y2024m12d10`; `Independently Solved`; `13ms`; `6.6mb`; `2 attempts`;

pub fn maximum_length(s: String) -> i32 {
    const ARRAY_REPEAT_VALUE: std::vec::Vec<usize> = Vec::new();
    let mut substr_count_vec_arr: [Vec<usize>; 26] = [ARRAY_REPEAT_VALUE; 26];
    let mut ans_count: i32 = -1;
    let mut s_char_iter = s.char_indices();
    let (mut prev_i, mut prev_char) = s_char_iter.next().expect("len >= 3");
    fn update_count_and_ans(
        substr_count_vec_mut_ref: &mut Vec<usize>,
        ans_count_mut_ref: &mut i32,
        substr_len: usize,
    ) {
        if substr_len > substr_count_vec_mut_ref.len() {
            substr_count_vec_mut_ref.reserve(substr_len - substr_count_vec_mut_ref.len());
        }
        while substr_count_vec_mut_ref.len() < substr_len {
            substr_count_vec_mut_ref.push(0);
        }
        for (i, entry_mut_ref) in substr_count_vec_mut_ref
            .iter_mut()
            .take(substr_len)
            .enumerate()
        {
            *entry_mut_ref += substr_len - i;
            if *entry_mut_ref >= 3 {
                *ans_count_mut_ref = (*ans_count_mut_ref).max((i + 1) as i32);
            }
        }
    }
    const LOWER_A_U8: u8 = b'a';
    fn lower_char_to_i(c: char) -> usize {
        (c as u8 - LOWER_A_U8) as usize
    }
    for (curr_i, curr_char) in s_char_iter {
        if curr_char == prev_char {
            continue;
        }
        update_count_and_ans(
            substr_count_vec_arr
                .get_mut(lower_char_to_i(prev_char))
                .expect("a to z"),
            &mut ans_count,
            curr_i - prev_i,
        );

        prev_char = curr_char;
        prev_i = curr_i;
    }
    update_count_and_ans(
        substr_count_vec_arr
            .get_mut(lower_char_to_i(prev_char))
            .expect("a to z"),
        &mut ans_count,
        s.len() - prev_i,
    );
    ans_count
}
