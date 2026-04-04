//! # LeetCode 2573. Find the String with Longest Common Prefix Matrix
//! https://leetcode.com/problems/find-the-string-with-lcp/
//! - y2026m03d28; Learned from Solution;
//! Learned from https://leetcode.com/problems/find-the-string-with-lcp/editorial

pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
    let len = lcp.len();

    let mut ans_string_c_u8_vec = vec![0; len];
    let mut c_u8 = b'a';

    for c_u8_idx in 0..len {
        if ans_string_c_u8_vec[c_u8_idx] != 0 {
            continue;
        }

        if c_u8 > b'z' {
            return String::new();
        }

        ans_string_c_u8_vec[c_u8_idx] = c_u8;

        let lcp_row_vec_ref = &lcp[c_u8_idx];
        for next_c_u8_idx in (c_u8_idx + 1)..len {
            if lcp_row_vec_ref[next_c_u8_idx] == 0 {
                continue;
            }

            let c_u8_entry_mut_ref = &mut ans_string_c_u8_vec[next_c_u8_idx];

            if (*c_u8_entry_mut_ref != 0) && (*c_u8_entry_mut_ref != c_u8) {
                return String::new();
            }

            *c_u8_entry_mut_ref = c_u8;
        }

        c_u8 += 1;
    }

    for i in (0..len).rev() {
        for j in (0..len).rev() {
            if ans_string_c_u8_vec[i] != ans_string_c_u8_vec[j] {
                if lcp[i][j] > 0 {
                    return String::new();
                }
            } else {
                if i == len - 1 || j == len - 1 {
                    if lcp[i][j] != 1 {
                        return String::new();
                    }
                } else {
                    if lcp[i][j] != lcp[i + 1][j + 1] + 1 {
                        return String::new();
                    }
                }
            }
        }
    }

    unsafe { String::from_utf8_unchecked(ans_string_c_u8_vec) }
}
