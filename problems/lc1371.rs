//! ## Leetcode 1371. Find the Longest Substring Containing Vowels in Even Counts
//! https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/
//! - `Medium`; `Independently Solved`; `2024-09-15`

pub fn find_the_longest_substring(s: String) -> i32 {
    #[inline]
    fn get_vowel_char_idx(c: char) -> Option<usize> {
        match c {
            'a' => Some(0),
            'e' => Some(1),
            'i' => Some(2),
            'o' => Some(3),
            'u' => Some(4),
            _ => None,
        }
    }
    let mut all_vowels_count_even_bitflags: u8 = 0;
    let mut ans_len: usize = 0;
    let mut prev_info: [Option<usize>; 32] = [None; 32];
    for (i, c) in s.char_indices() {
        if let Some(vowel_i) = get_vowel_char_idx(c) {
            let pos_bitmask: u8 = 1 << vowel_i;
            all_vowels_count_even_bitflags ^= pos_bitmask;
        }
        let prev_cell_mut_ref = &mut prev_info[all_vowels_count_even_bitflags as usize];
        if let Some(prev_i_mut_ref) = prev_cell_mut_ref {
            ans_len = ans_len.max(i - *prev_i_mut_ref);
        } else {
            *prev_cell_mut_ref = Some(i);
        }
    }
    ans_len as i32
}
