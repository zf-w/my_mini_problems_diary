//! # Leetcode 717. 1-bit and 2-bit Characters
//! https://leetcode.com/problems/1-bit-and-2-bit-characters/
//! - `Easy`; `y2025m11d18`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn is_one_bit_character( mut bits: Vec<i32>) -> bool {
            bits.pop();
    let mut bit_flag_iter = bits.into_iter().map(|c: i32| -> bool { c == 1 });

    let mut prev_is_one_flag = false;

    while let Some(bit_flag) = bit_flag_iter.next() {
        prev_is_one_flag = prev_is_one_flag == false && bit_flag == true;
        // println!("{}", prev_is_one_flag);
    }

    prev_is_one_flag == false
}