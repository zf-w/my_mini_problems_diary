//! # Leetcode 2900. Longest Unequal Adjacent Groups Subsequence I
//! https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-i/
//! - `Easy`; `y2025m05d15`; `Independently Solved`; `0ms`; `2.3mb`; `3 attempts`;
//! Topics subsequence.

pub fn get_longest_subsequence(mut words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    let len = groups.len();

    let mut group_iter = groups.into_iter().map(|v| -> bool { v == 0 }).enumerate();

    let mut ans_vec: Vec<String> = Vec::with_capacity(len);

    let mut push_closure = |i: usize| {
        let mut to_swap_string = String::new();

        std::mem::swap(&mut to_swap_string, &mut words[i]);
        ans_vec.push(to_swap_string);
    };

    let (_, mut prev_flag) = group_iter.next().expect("len > 0");

    push_closure(0);

    for (i, flag) in group_iter {
        if prev_flag != flag {
            // println!("{} {} {} {}", begin_i, end_i, ans_begin_i, ans_end_i);
            push_closure(i);
        }

        prev_flag = flag;
    }

    ans_vec
}
