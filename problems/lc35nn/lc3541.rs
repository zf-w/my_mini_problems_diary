//! # Leetcode 3541. Find Most Frequent Vowel and Consonant
//! https://leetcode.com/problems/find-most-frequent-vowel-and-consonant/
//! - `Easy`; `y2025m09d12`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: count_tracking.

pub fn max_freq_sum(s: String) -> i32 {
    const LETTER_NUM: usize = 26;
    const LETTER_LOWER_A_IDX: usize = b'a' as usize;
    const VOWEL_IDX_ARR: [usize; 5] = [
        'a' as u8 as usize - LETTER_LOWER_A_IDX,
        'e' as u8 as usize - LETTER_LOWER_A_IDX,
        'i' as u8 as usize - LETTER_LOWER_A_IDX,
        'o' as u8 as usize - LETTER_LOWER_A_IDX,
        'u' as u8 as usize - LETTER_LOWER_A_IDX,
    ];
    let mut freq_count_arr: [usize; LETTER_NUM] = [0; LETTER_NUM];

    for c in s.chars() {
        let c_idx = (c as u8 - b'a') as usize;

        freq_count_arr[c_idx] += 1;
    }

    let (vowel_max, conso_max) = freq_count_arr.into_iter().enumerate().fold(
        (0, 0),
        |(sum_0, sum_1), (c_idx, count)| -> (usize, usize) {
            // println!("{} {}",(b'a' + (c_idx as usize as u8)) as char, count);
            if VOWEL_IDX_ARR.contains(&c_idx) {
                (sum_0.max(count), sum_1)
            } else {
                (sum_0, sum_1.max(count))
            }
        },
    );

    (vowel_max + conso_max) as i32
}
