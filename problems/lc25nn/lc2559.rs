//! # Leetcode 2559. Count Vowel Strings in Ranges
//! https://leetcode.com/problems/count-vowel-strings-in-ranges/
//! - `Medium`; `y2025m01d02`; `Independently Solved`; `4ms`; `14.6mb`; `2 attempts`;

const VOWEL_CHAR_U8_ARR: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];

pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans_vec: Vec<i32> = Vec::with_capacity(queries.len());
    let mut prefix_sum: Vec<usize> = Vec::with_capacity(words.len() + 1);
    prefix_sum.push(0);
    let mut curr_count: usize = 0;
    for word in words {
        let start_c = word.as_bytes().first().cloned().expect("len > 0");
        let end_c = word.as_bytes().last().cloned().expect("len > 0");
        if VOWEL_CHAR_U8_ARR.contains(&end_c) && VOWEL_CHAR_U8_ARR.contains(&start_c) {
            curr_count += 1;
        }
        prefix_sum.push(curr_count);
    }

    for query in queries {
        let begin_i = query[0] as usize;
        let end_i = query[1] as usize + 1;
        ans_vec.push((prefix_sum[end_i] - prefix_sum[begin_i]) as i32);
    }
    ans_vec
}
