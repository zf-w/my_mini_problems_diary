//! # Leetcode 1718. Construct the Lexicographically Largest Valid Sequence
//! https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/
//! - `Medium`; `y2025m02d16`; `Independently Solved`; `0ms`; `2.32mb`; `1 attempt`;

pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    let num_num = (n + 1) as usize;
    let len = (n * 2 - 1) as usize;
    let mut usage_flag_vec: Vec<bool> = vec![false; num_num];

    let placeholder_num = n + 1;
    let mut ans_vec = vec![placeholder_num; len];

    fn try_fill(
        idx: usize,
        n: i32,
        ans_vec_mut_ref: &mut [i32],
        usage_flag_vec_mut_ref: &mut [bool],
    ) -> bool {
        let placeholder_num = n + 1;
        let len = ans_vec_mut_ref.len();
        for to_try_num in (1..=n).rev() {
            let num_idx: usize = to_try_num as usize;
            let sep_len: usize = num_idx - 1;
            if usage_flag_vec_mut_ref[num_idx] == true {
                continue;
            }

            if idx + sep_len >= len || ans_vec_mut_ref[idx + sep_len] != placeholder_num {
                continue;
            }

            let mut next_idx = idx + 1;

            while next_idx < len && ans_vec_mut_ref[next_idx] != placeholder_num {
                next_idx += 1;
            }

            if next_idx < len {
                usage_flag_vec_mut_ref[num_idx] = true;
                ans_vec_mut_ref[idx] = to_try_num;
                ans_vec_mut_ref[idx + sep_len] = to_try_num;
                if try_fill(next_idx, n, ans_vec_mut_ref, usage_flag_vec_mut_ref) {
                    return true;
                }

                usage_flag_vec_mut_ref[num_idx] = false;
                ans_vec_mut_ref[idx] = placeholder_num;
                ans_vec_mut_ref[idx + sep_len] = placeholder_num;
            }
        }
        false
    }

    try_fill(0, n, &mut ans_vec, &mut usage_flag_vec);

    ans_vec
}
