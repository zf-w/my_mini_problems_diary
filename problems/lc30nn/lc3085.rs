//! # Leetcode 3085. Minimum Deletions to Make String K-Special
//! https://leetcode.com/problems/minimum-deletions-to-make-string-k-special/
//! - `Medium`; `y2025m06d21`; `Learned from Solution`; `0ms`; `2.4mb`; `1 attempt`;
//! Topics: count_tracking.
//!
//! I didn't think of deleting a low-count character hahahaha.

pub fn minimum_deletions(word: String, k: i32) -> i32 {
    let mut count_arr: [usize; 26] = [0; 26];

    for c in word.chars() {
        count_arr[(c as u8 - b'a') as usize] += 1;
    }

    let mut ans_min_del = word.len();

    let k = k as usize;

    for i in 0..26 {
        if count_arr[i] == 0 {
            continue;
        }

        let i_count = count_arr[i];
        let i_bound = i_count + k;

        let mut del_num: usize = 0;

        for j in 0..26usize {
            if count_arr[j] == 0 || i == j {
                continue;
            }

            let j_count = count_arr[j];

            if j_count > i_bound {
                del_num += j_count - i_bound;
            } else if j_count < i_count {
                del_num += j_count;
            }
        }
        ans_min_del = ans_min_del.min(del_num);
    }
    ans_min_del as i32
}
