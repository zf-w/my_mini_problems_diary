//! ## Leetcode 1593. Split a String Into the Max Number of Unique Substrings
//! https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/
//! - `Medium`; `Independently Solved`; `y2024m10d21`;

fn max_unique_split_helper<'src>(
    level: usize,
    s_str_bytes_ref: &'src [u8],
    mp_mut_ref: &mut std::collections::HashSet<&'src [u8]>,
    ans_len_mut_ref: &mut usize,
) {
    let len = s_str_bytes_ref.len();
    let next_level = level + 1;
    for i in 1..=len {
        let curr_seg_str_bytes_ref = &s_str_bytes_ref[..i];
        if mp_mut_ref.contains(curr_seg_str_bytes_ref) {
            continue;
        }
        // println!("{} {}", next_level, unsafe {
        //         String::from_utf8_unchecked(curr_seg_str_bytes_ref.to_vec())
        //     });
        if i == len {
            *ans_len_mut_ref = (*ans_len_mut_ref).max(next_level);
        } else {
            mp_mut_ref.insert(curr_seg_str_bytes_ref);
            max_unique_split_helper(
                next_level,
                &s_str_bytes_ref[i..],
                mp_mut_ref,
                ans_len_mut_ref,
            );
            mp_mut_ref.remove(curr_seg_str_bytes_ref);
        }
    }
}

pub fn max_unique_split(s: String) -> i32 {
    let mut ans_len: usize = 0;
    let mut mp: std::collections::HashSet<&[u8]> = std::collections::HashSet::new();
    max_unique_split_helper(0, s.as_bytes(), &mut mp, &mut ans_len);
    ans_len as i32
}
