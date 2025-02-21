//! # Leetcode 2375. Construct Smallest Number From DI String
//! https://leetcode.com/problems/construct-smallest-number-from-di-string/
//! - `Medium`; `y2025m02d20`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;
//!
//! Topic: recursion.

pub fn smallest_number(pattern: String) -> String {
    let pattern_up_bool_iter = pattern
        .as_bytes()
        .iter()
        .map(|c_u8| -> bool { *c_u8 == b'I' });

    let mut ans_u8_vec: Vec<u8> = Vec::with_capacity(pattern.len() + 1);

    fn helper(
        ans_u8_vec_mut_ref: &mut Vec<u8>,
        mut pattern_up_bool_iter: impl Iterator<Item = bool> + Clone,
        used_mask_mut_ref: &mut u16,
    ) -> bool {
        let curr_up_bool: bool = if let Some(curr_up_bool) = pattern_up_bool_iter.next() {
            curr_up_bool
        } else {
            return true;
        };

        let curr_num = ans_u8_vec_mut_ref.last().expect("Should have one").clone();

        if curr_up_bool == true {
            if curr_num == 9 {
                unreachable!();
            }
            for next_num in (curr_num + 1)..10 {
                if (*used_mask_mut_ref >> next_num) & 1 == 1 {
                    continue;
                }

                *used_mask_mut_ref |= 1 << next_num;

                ans_u8_vec_mut_ref.push(next_num);

                if helper(
                    ans_u8_vec_mut_ref,
                    pattern_up_bool_iter.clone(),
                    used_mask_mut_ref,
                ) {
                    return true;
                }

                ans_u8_vec_mut_ref.pop();
                *used_mask_mut_ref ^= 1 << next_num;
            }
        } else {
            if curr_num == 1 {
                return false;
            }
            for next_num in 1..curr_num {
                if (*used_mask_mut_ref >> next_num) & 1 == 1 {
                    continue;
                }

                *used_mask_mut_ref |= 1 << next_num;

                ans_u8_vec_mut_ref.push(next_num);

                if helper(
                    ans_u8_vec_mut_ref,
                    pattern_up_bool_iter.clone(),
                    used_mask_mut_ref,
                ) {
                    return true;
                }

                ans_u8_vec_mut_ref.pop();

                *used_mask_mut_ref ^= 1 << next_num;
            }
        }
        false
    }

    let mut used_mask: u16 = 0;

    for first_num in 1..10 {
        used_mask |= 1 << first_num;

        ans_u8_vec.push(first_num);

        if helper(
            &mut ans_u8_vec,
            pattern_up_bool_iter.clone(),
            &mut used_mask,
        ) {
            break;
        }

        used_mask ^= 1 << first_num;

        ans_u8_vec.pop();
    }

    for byte_mut_ref in ans_u8_vec.iter_mut() {
        *byte_mut_ref += b'0';
    }

    String::from_utf8(ans_u8_vec).expect("Should work.")
}
