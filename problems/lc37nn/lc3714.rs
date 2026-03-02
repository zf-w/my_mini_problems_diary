//! # Leetcode 3714. Longest Balanced Substring II
//! https://leetcode.com/problems/longest-balanced-substring-ii/
//! - y2026m02d13; Learned from Solution;
//! 
//! Super slow hahaha.

pub fn longest_balanced(s: String) -> i32 {
    use std::collections::hash_map::{Entry, HashMap};

    fn longest_balanced_1(byte_arr_ref: &[u8]) -> usize {
        let mut byte_iter = byte_arr_ref.iter().cloned();
        let mut prev_byte = byte_iter.next().expect("len > 0");

        let mut ans_max_len = 1;
        let mut curr_len = 1;

        for byte in byte_iter {
            if byte == prev_byte {
                curr_len += 1;
            } else {
                ans_max_len = ans_max_len.max(curr_len);
                curr_len = 1;
            }

            prev_byte = byte;
        }

        ans_max_len.max(curr_len)
    }

    fn longest_balanced_2(
        byte_arr_ref: &[u8],
        c_0: u8,
        c_1: u8,
        map_mut_ref: &mut HashMap<i32, usize>,
    ) -> usize {
        let mut c_0_count: i32 = 0;
        let mut c_1_count: i32 = 0;

        map_mut_ref.insert(0, 0);

        let mut ans_len: usize = 0;

        for (i, byte) in byte_arr_ref.iter().cloned().enumerate() {
            let end_i = i + 1;

            if byte == c_0 {
                c_0_count += 1;
            } else if byte == c_1 {
                c_1_count += 1;
            } else {
                map_mut_ref.clear();
                map_mut_ref.insert(0, end_i);
                c_0_count = 0;
                c_1_count = 0;
                continue;
            }

            let diff_key = c_1_count - c_0_count;
            // println!("({} {}) {}", c_0, c_1, diff_key);
            match map_mut_ref.entry(diff_key) {
                Entry::Occupied(occupied_entry) => {
                    let prev_i = occupied_entry.get().clone();
                    ans_len = ans_len.max(end_i - prev_i);
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(end_i);
                }
            }
        }

        map_mut_ref.clear();

        ans_len
    }

    let byte_arr_ref = s.as_bytes();
    let mut ans_max_len = longest_balanced_1(byte_arr_ref);

    let mut diff_key_map: HashMap<i32, usize> = HashMap::with_capacity(byte_arr_ref.len());

    ans_max_len = ans_max_len.max(longest_balanced_2(
        byte_arr_ref,
        b'a',
        b'b',
        &mut diff_key_map,
    ));

    ans_max_len = ans_max_len.max(longest_balanced_2(
        byte_arr_ref,
        b'b',
        b'c',
        &mut diff_key_map,
    ));

    ans_max_len = ans_max_len.max(longest_balanced_2(
        byte_arr_ref,
        b'a',
        b'c',
        &mut diff_key_map,
    ));

    println!("{}", ans_max_len);

    let mut diff_2_map: HashMap<(i32, i32), usize> = HashMap::with_capacity(byte_arr_ref.len());

    let mut c_0_count: i32 = 0;
    let mut c_1_count: i32 = 0;
    let mut c_2_count: i32 = 0;

    diff_2_map.insert((0, 0), 0);

    for (i, byte) in byte_arr_ref.iter().cloned().enumerate() {
        let end_i = i + 1;

        if byte == b'a' {
            c_0_count += 1;
        } else if byte == b'b' {
            c_1_count += 1;
        } else {
            c_2_count += 1;
        }

        let diff_key = (c_2_count - c_0_count, c_2_count - c_1_count);

        match diff_2_map.entry(diff_key) {
            Entry::Occupied(occupied_entry) => {
                let prev_i = occupied_entry.get().clone();
                ans_max_len = ans_max_len.max(end_i - prev_i);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(end_i);
            }
        }
    }

    ans_max_len as i32
}