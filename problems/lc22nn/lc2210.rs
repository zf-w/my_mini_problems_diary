//! Leetcode 2210. Count Hills and Valleys in an Array
//! https://leetcode.com/problems/count-hills-and-valleys-in-an-array/
//! - `Easy`; `y2025m07d27`; `Independently Solved`; `0ms`; `2.2mb`; `2 attempts`;
//! Topics: uncategorized.

pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
    let mut prev_info_opt: Option<bool> = None;
    let mut iter = nums.iter().cloned();
    let mut prev_num = iter.next().expect("len > 0");
    let mut curr_num = iter.next().expect("len > 1");

    if curr_num > prev_num {
        prev_info_opt = Some(true);
    } else if curr_num < prev_num {
        prev_info_opt = Some(false);
    }

    let mut ans: usize = 0;

    for next_num in iter {
        if curr_num < next_num {
            if prev_num == curr_num {
                if let Some((up_flag_mut_ref)) = prev_info_opt.as_mut() {
                    if *up_flag_mut_ref == false {
                        ans += 1;
                    }
                }
            } else if prev_num > curr_num {
                ans += 1;
            }
            prev_info_opt = Some(true);
        } else if curr_num > next_num {
            if prev_num == curr_num {
                if let Some((up_flag_mut_ref)) = prev_info_opt.as_mut() {
                    if *up_flag_mut_ref == true {
                        ans += 1;
                    }
                }
            } else if prev_num < curr_num {
                ans += 1;
            }
            prev_info_opt = Some(false);
        }

        prev_num = curr_num;
        curr_num = next_num;
    }

    ans as i32
}