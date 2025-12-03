//! # Leetcode 474. Ones and Zeros
//! https://leetcode.com/problems/ones-and-zeroes/
//! - `Medium`; `y2025m11d10`; `Independently Solved`; `50ms`; `54.8mb`; `1 attempt`;
//! Topics: dynamic_programming.

pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut info_vec: Vec<(usize, usize)> = Vec::with_capacity(strs.len());

        fn info_fold_fn((zero_num, one_num): (usize, usize), c: char) -> (usize, usize) {
            if c == '0' {
                (zero_num + 1, one_num)
            } else {
                (zero_num, one_num + 1)
            }
        }

        for str in strs {
            info_vec.push(str.chars().fold((0, 0), info_fold_fn))
        }

        let zero_limit = m as usize;
        let one_limit = n as usize;

        let zero_end = zero_limit + 1;
        let one_end = one_limit + 1;
        let len = info_vec.len();
        let len_end = len + 1;

        let index_fn = |idx: usize, zero_limit: usize, one_limit: usize| -> usize {
            idx * (zero_end * one_end) + zero_limit * one_end + one_limit
        };

        let mut memo_table = vec![len_end; len * zero_end * one_end].into_boxed_slice();

           fn search_fn(
            idx: usize,
            zero_limit: usize,
            one_limit: usize,
            memo_table_mut_ref: &mut [usize],
            static_info_pack: (&impl Fn(usize, usize, usize) -> usize, &[(usize, usize)]),
        ) -> usize {
            let (index_fn_ref, info_arr_ref) = static_info_pack;

            if idx >= info_arr_ref.len() {
                return 0;
            }

            let len_end = info_arr_ref.len() + 1;

            let memo_entry_idx = index_fn_ref(idx, zero_limit, one_limit);

            let memo_value = memo_table_mut_ref[memo_entry_idx];

            if memo_value < len_end {
                return memo_value;
            }

            let (curr_zero_num, curr_one_num) = info_arr_ref[idx];

            let mut ans_num = search_fn(
                idx + 1,
                zero_limit,
                one_limit,
                memo_table_mut_ref,
                static_info_pack,
            );

            if curr_zero_num <= zero_limit && curr_one_num <= one_limit {
                ans_num = ans_num.max(1 + search_fn(
                    idx + 1,
                    zero_limit - curr_zero_num,
                    one_limit - curr_one_num,
                    memo_table_mut_ref,
                    static_info_pack,
                ));
            }

            memo_table_mut_ref[memo_entry_idx] = ans_num;

            ans_num
        }

        search_fn(
            0,
            zero_limit,
            one_limit,
            &mut memo_table,
            (&index_fn, &info_vec),
        ) as i32
    }