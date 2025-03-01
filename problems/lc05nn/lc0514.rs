//! ## Leetcode 514. Freedom Trail
//! https://leetcode.com/problems/freedom-trail
//! - `Hard`; `Learned from Solution`; `2024-04-27`;
//!
//! We need to apply dynamic programming on the location of the currect target character and the location of previous target character on the lock. One interesting technique I applied is using an index function to swap between the DP data of the previous information and the current information, without additional copying of data.

pub fn find_rotate_steps(ring: String, key: String) -> i32 {
    let c_base = 'a' as u8 as usize;
    let char_to_usize = |c: char| -> usize { c as u8 as usize - c_base };

    let mut key_map: [Vec<usize>; 26] =
        std::array::from_fn(|_| -> Vec<usize> { Vec::with_capacity(100) });
    for (i, c) in ring.chars().map(char_to_usize).enumerate() {
        key_map[c].push(i);
    }

    let ring_len = ring.len();
    let mut dp: Vec<usize> = vec![0; ring_len * 2];
    let index = |first: bool, i: usize| -> usize {
        if first {
            i
        } else {
            ring_len + i
        }
    };
    let mut curr_part = true;
    let mut key_iter = key.chars().map(char_to_usize);
    let mut prev_key = key_iter.next().expect("key length greater than 1");

    let min_dis_between_key = |pos_0: &usize, pos_1: &usize| -> usize {
        if pos_0 > pos_1 {
            (pos_0 - pos_1).min(ring_len - pos_0 + pos_1)
        } else {
            (pos_1 - pos_0).min(ring_len - pos_1 + pos_0)
        }
    };
    let mut min_dis: Option<usize> = None;
    for key_pos in key_map[prev_key].iter() {
        let curr_dp_i = index(curr_part, *key_pos);
        dp[curr_dp_i] = (*key_pos).min(ring_len - *key_pos);
        min_dis = match min_dis {
            None => Some(dp[curr_dp_i]),
            Some(v) => Some(v.min(dp[curr_dp_i])),
        };
    }

    for curr_key in key_iter {
        curr_part = !curr_part;
        min_dis = None;
        for curr_pos in key_map[curr_key].iter() {
            let curr_dp_i = index(curr_part, *curr_pos);
            let mut prev_pos_iter = key_map[prev_key].iter();
            let first_prev_pos = prev_pos_iter.next().expect("Should be able to spell");
            dp[curr_dp_i] = min_dis_between_key(first_prev_pos, curr_pos)
                + dp[index(!curr_part, *first_prev_pos)];
            for prev_pos in prev_pos_iter {
                dp[curr_dp_i] = dp[curr_dp_i].min(
                    min_dis_between_key(curr_pos, prev_pos) + dp[index(!curr_part, *prev_pos)],
                );
            }
            min_dis = match min_dis {
                None => Some(dp[curr_dp_i]),
                Some(v) => Some(v.min(dp[curr_dp_i])),
            };
        }
        prev_key = curr_key;
    }
    min_dis.expect("Should have") as i32
}
