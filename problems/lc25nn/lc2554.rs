//! # Leetcode 2554. Maximum Number of Integers to Choose From a Range I
//! https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-i/
//! - `Medium`; `y2024m12d06`; `Independently Solved`; `23ms`; `2.4mb`; `3 attempts`;

pub fn max_count(mut banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
    banned.push(n + 1);
    banned.sort_unstable();
    let mut prev_banned_num = 0;
    let mut curr_sum = 0;
    let mut ans_count = 0;
    for banned_num in banned.into_iter() {
        // println!("{} {}", curr_sum, banned_num);
        if banned_num == prev_banned_num {
            continue;
        }
        let elem_num = banned_num - prev_banned_num - 1;
        let group_add_num = (prev_banned_num + banned_num) * (elem_num) / 2;
        if curr_sum + group_add_num > max_sum {
            for elem in (prev_banned_num + 1)..banned_num {
                if curr_sum + elem > max_sum {
                    return ans_count;
                }
                curr_sum += elem;
                ans_count += 1;
            }
        } else if curr_sum + group_add_num == max_sum {
            return ans_count + elem_num;
        }
        ans_count += elem_num;
        curr_sum += group_add_num;
        prev_banned_num = banned_num;
        if banned_num == n + 1 {
            return ans_count;
        }
    }
    ans_count
}
