//! # Leetcode 916. Word Subsets
//! https://leetcode.com/problems/word-subsets/
//! - `Medium`; `y2025m01d10`; `Independently Solved`; `5ms`; `3.7mb`; `1 attempt`;

pub fn word_subsets(mut words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    const CHAR_LOWER_A_U8: u8 = b'a';
    let mut char_requirement: [usize; 26] = [0; 26];

    #[inline]
    fn calc_char_idx(c_u8: u8) -> usize {
        (c_u8 - CHAR_LOWER_A_U8) as usize
    }

    #[inline]
    fn count_char_freq(c_u8_iter: impl Iterator<Item = u8>) -> [usize; 26] {
        let mut curr_char_count: [usize; 26] = [0; 26];
        for c_u8 in c_u8_iter {
            let c_idx = calc_char_idx(c_u8);
            curr_char_count[c_idx] += 1;
        }
        curr_char_count
    }

    for word_2 in words2 {
        let curr_char_count = count_char_freq(word_2.as_bytes().iter().cloned());
        for (req_mut_ref, curr_count) in char_requirement
            .iter_mut()
            .zip(curr_char_count.iter().cloned())
        {
            *req_mut_ref = (*req_mut_ref).max(curr_count);
        }
    }

    let mut word_1_i: usize = 0;

    let mut last_i = words1.len() - 1;

    while let Some(word_1_ref) = words1.get(word_1_i) {
        let curr_char_count = count_char_freq(word_1_ref.as_bytes().iter().cloned());

        let mut is_subset_flag = true;
        for (req_count, curr_count) in char_requirement
            .iter()
            .cloned()
            .zip(curr_char_count.iter().cloned())
        {
            if req_count > curr_count {
                // First bug here. Was < before haha.
                is_subset_flag = false;
                break;
            }
        }
        if is_subset_flag == false {
            words1.swap(word_1_i, last_i);
            words1.pop();
            last_i -= 1;
        } else {
            word_1_i += 1;
        }
    }
    words1
}
