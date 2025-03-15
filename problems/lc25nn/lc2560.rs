//! # Leetcode 2560. House Robber IV
//! https://leetcode.com/problems/house-robber-iv/
//! - `Medium`; `y2025m03d14`; `Learned from Solution`; `15ms`; `4mb`; `3 attempts`;
//! Topics: binary_search.
//! Learned from https://leetcode.com/problems/house-robber-iv/editorial
//!
//! I was indeed thinking in a very strange way at first.

pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let k = k as usize;
    let mut num_iter = nums.iter().cloned();
    let mut begin_num = num_iter.next().expect("len > 0");
    let mut end_num = begin_num;
    for num in num_iter {
        begin_num = begin_num.min(num);
        end_num = end_num.max(num);
    }

    end_num += 1;

    while begin_num < end_num {
        let mid_num = (begin_num + end_num) / 2;

        let mut i: usize = 0;
        let mut pick_num: usize = 0;
        while i < len {
            if nums[i] <= mid_num {
                pick_num += 1;
                i += 1;
            }
            if pick_num >= k {
                break;
            }
            i += 1;
        }

        if pick_num >= k {
            end_num = mid_num;
        } else {
            begin_num = mid_num + 1;
        }
        // println!("{} {}", begin_num, end_num);
    }

    begin_num
}

pub fn min_capability_1(mut nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut idx_arr_box: Box<[usize]> = vec![0; len].into_boxed_slice();
    for (i, entry_mut_ref) in idx_arr_box.iter_mut().enumerate() {
        *entry_mut_ref = i;
    }

    idx_arr_box.sort_unstable_by_key(|idx| -> i32 { nums[*idx] });

    // println!("{:?}", idx_arr_box);

    let mut begin_i = 0;
    let mut end_i = len;

    fn check_idx_range(idx_arr_ref: &[usize], num_arr_mut_ref: &mut [i32], k: i32) -> bool {
        if k == 0 {
            return true;
        }

        if idx_arr_ref.is_empty() {
            return false;
        }

        let to_pick_or_not_to_pick_idx = idx_arr_ref[0];
        let mut can_pick_flag = true;

        if to_pick_or_not_to_pick_idx > 0 && num_arr_mut_ref[to_pick_or_not_to_pick_idx - 1] < 0 {
            can_pick_flag = false;
        }

        if can_pick_flag
            && to_pick_or_not_to_pick_idx + 1 < num_arr_mut_ref.len()
            && num_arr_mut_ref[to_pick_or_not_to_pick_idx + 1] < 0
        {
            can_pick_flag = false;
        }

        let mut res = false;
        if can_pick_flag {
            let curr_num = num_arr_mut_ref[to_pick_or_not_to_pick_idx];
            num_arr_mut_ref[to_pick_or_not_to_pick_idx] = -curr_num;
            res |= check_idx_range(&idx_arr_ref[1..], num_arr_mut_ref, k - 1);
            num_arr_mut_ref[to_pick_or_not_to_pick_idx] = curr_num;
        }

        if res {
            return true;
        }

        res |= check_idx_range(&idx_arr_ref[1..], num_arr_mut_ref, k);

        res
    }

    while begin_i < end_i {
        let mid_idx_i = (begin_i + end_i) / 2;
        // let curr_idx = idx_arr_box[mid_idx_i];
        // let curr_num = nums[curr_idx];
        // nums[curr_idx] = -curr_num;
        println!("{} {}", begin_i, end_i);
        if check_idx_range(&idx_arr_box[..=mid_idx_i], &mut nums, k) {
            end_i = mid_idx_i;
        } else {
            begin_i = mid_idx_i + 1;
        }
        // nums[curr_idx] = curr_num;
    }

    nums[idx_arr_box[begin_i]]
}
