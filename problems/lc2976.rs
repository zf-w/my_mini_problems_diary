//! ## Leetcode 2976. Minimum Cost to Convert String I
//! https://leetcode.com/problems/minimum-cost-to-convert-string-i/
//! - `Medium`; `Independently Solved`; `2024-07-27`;

const LOWER_A_U8: u8 = 'a' as u8;
const LEN: usize = 26;

pub fn minimum_cost(
    source: String,
    target: String,
    original: Vec<char>,
    changed: Vec<char>,
    cost: Vec<i32>,
) -> i64 {
    let mut dis_adj: [Option<i64>; LEN * LEN] = [None; LEN * LEN];
    fn char_to_idx_fn(c: char) -> usize {
        (c as u8 - LOWER_A_U8) as usize
    }
    fn index_fn((i_0, i_1): (usize, usize)) -> usize {
        i_0 * LEN + i_1
    }

    for i in 0..LEN {
        dis_adj[index_fn((i, i))] = Some(0);
    }

    for ((from_char, to_char), cost_i32) in original
        .iter()
        .cloned()
        .zip(changed.iter().cloned())
        .zip(cost.iter().cloned())
    {
        let i_0 = char_to_idx_fn(from_char);
        let i_1 = char_to_idx_fn(to_char);
        dis_adj[index_fn((i_0, i_1))] = Some(cost_i32 as i64);
    }

    for i_1 in 0..LEN {
        for i_0 in 0..LEN {
            if i_0 == i_1 {
                continue;
            }
            for i_2 in 0..LEN {
                if i_2 == i_0 || i_2 == i_1 {
                    continue;
                }
                let entry_i = index_fn((i_0, i_2));
                if let (Some(part_0_dis), Some(part_1_dis)) =
                    (dis_adj[index_fn((i_0, i_1))], dis_adj[index_fn((i_1, i_2))])
                {
                    let route_pass_node_1_dis = part_0_dis + part_1_dis;
                    if let Some(curr_dis_mut_ref) = dis_adj[entry_i].as_mut() {
                        *curr_dis_mut_ref = (*curr_dis_mut_ref).min(route_pass_node_1_dis);
                    } else {
                        dis_adj[entry_i] = Some(route_pass_node_1_dis);
                    }
                }
            }
        }
    }
    let mut ans_dis_sum = 0;
    for dis_entry_i in source
        .chars()
        .map(char_to_idx_fn)
        .zip(target.chars().map(char_to_idx_fn))
        .map(index_fn)
    {
        if let Some(curr_dis) = dis_adj[dis_entry_i] {
            ans_dis_sum += curr_dis;
        } else {
            return -1;
        }
    }

    ans_dis_sum
}
