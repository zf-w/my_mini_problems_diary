//! # Leetcode 1061. Lexicographically Smallest Equivalent String
//! https://leetcode.com/problems/lexicographically-smallest-equivalent-string/
//! - `Medium`; `y2025m06d05`; `Independently Solved`; `0ms`; `2.1mb`; `1 attempt`;
//! Topics: union_find.

pub fn smallest_equivalent_string(s1: String, s2: String, mut base_str: String) -> String {
    let s1_char_arr_ref = s1.as_bytes();
    let s2_char_arr_ref = s2.as_bytes();

    let mut mapping_idx_arr: [usize; 26] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25,
    ];

    fn mapping_find(mapping_idx_arr_mut_ref: &mut [usize; 26], idx: usize) -> usize {
        let next_idx = mapping_idx_arr_mut_ref[idx];
        if next_idx == idx {
            return next_idx;
        }

        let ans_idx = mapping_find(mapping_idx_arr_mut_ref, next_idx);

        mapping_idx_arr_mut_ref[idx] = ans_idx;

        ans_idx
    }

    fn mapping_union(
        mapping_idx_arr_mut_ref: &mut [usize; 26],
        idx_0: usize,
        idx_1: usize,
    ) -> bool {
        let ans_idx_0 = mapping_find(mapping_idx_arr_mut_ref, idx_0);
        let ans_idx_1 = mapping_find(mapping_idx_arr_mut_ref, idx_1);

        if ans_idx_0 == ans_idx_1 {
            return false;
        }

        if ans_idx_1 < ans_idx_0 {
            mapping_idx_arr_mut_ref[ans_idx_0] = ans_idx_1;
        } else {
            mapping_idx_arr_mut_ref[ans_idx_1] = ans_idx_0;
        }
        true
    }

    fn char_u8_ref_to_idx(c_u8_ref: &u8) -> usize {
        (*c_u8_ref - b'a') as usize
    }

    for (c_0_idx, c_1_idx) in s1_char_arr_ref
        .iter()
        .map(char_u8_ref_to_idx)
        .zip(s2_char_arr_ref.iter().map(char_u8_ref_to_idx))
    {
        mapping_union(&mut mapping_idx_arr, c_0_idx, c_1_idx);
    }

    for c in unsafe { base_str.as_bytes_mut() } {
        *c = mapping_find(&mut mapping_idx_arr, (*c - b'a') as usize) as u8 + b'a';
    }

    base_str
}
