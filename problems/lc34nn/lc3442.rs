//! # Leetcode 3442. Maximum Difference Between Even and Odd Frequency I
//! https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-i/
//! - `Easy`; `y2025m06d10`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn max_difference(s: String) -> i32 {
    let mut char_freq_arr: [usize; 26] = [0; 26];

    for c in s.chars() {
        char_freq_arr[(c as u8 - b'a') as usize] += 1;
    }

    let mut max_odd_freq: Option<usize> = None;
    let mut min_even_freq: Option<usize> = None;

    for freq in char_freq_arr.into_iter() {
        if freq & 1 == 1 {
            if let Some(v_mut_ref) = max_odd_freq.as_mut() {
                *v_mut_ref = (*v_mut_ref).max(freq)
            } else {
                max_odd_freq = Some(freq);
            }
        } else {
            if let Some(v_mut_ref) = min_even_freq.as_mut() {
                *v_mut_ref = (*v_mut_ref).min(freq)
            } else {
                min_even_freq = Some(freq);
            }
        }
    }

    (max_odd_freq.expect("should have...") - min_even_freq.expect("should have...")) as i32
}
