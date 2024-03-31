//! ## Leetcode 2444. Count Subarrays With Fixed Bounds
//! https://leetcode.com/problems/count-subarrays-with-fixed-bounds
//! - `Hard`; `Independently Solved`; `2024-03-31`;
//!
//! I'm trying to make every outer loop a valid starting point instead of storing extra information about whether the current state is a start of numbers between `minK` and `maxK`.

pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let len = nums.len();
    let mut begin_i: usize;
    let mut end_i: usize = 1;
    while end_i < len && (nums[end_i] > max_k || nums[end_i] < min_k) {
        end_i += 1;
    }
    begin_i = end_i;
    end_i += 1;
    let mut curr_min = nums[begin_i];
    let mut curr_max = nums[begin_i];
    let mut last_min_i = if nums[begin_i] == min_k { begin_i } else { len };
    let mut last_max_i = if nums[begin_i] == max_k { begin_i } else { len };

    let mut ans = 0;

    let update = |curr_i: usize, last_max_i: &mut usize, last_min_i: &mut usize| {
        let curr = nums[curr_i];
        if curr == min_k {
            *last_min_i = curr_i
        }

        if curr == max_k {
            *last_max_i = curr_i
        }
    };

    let update_ans = |ans: &mut i64,
                      begin_i: &usize,
                      curr_max: &i32,
                      curr_min: &i32,
                      last_max_i: &usize,
                      last_min_i: &usize| {
        if *curr_min == min_k && *curr_max == max_k && last_max_i < &len && last_min_i < &len {
            let last_needed = if max_k == min_k {
                last_max_i.max(last_min_i)
            } else {
                last_max_i.min(last_min_i)
            };
            *ans += (last_needed + 1 - begin_i) as i64;
        }
    };

    update_ans(
        &mut ans,
        &begin_i,
        &curr_max,
        &curr_min,
        &last_max_i,
        &last_min_i,
    );
    while end_i < len {
        let curr = nums[end_i];
        if curr > max_k || curr < min_k {
            while end_i < len && (nums[end_i] > max_k || nums[end_i] < min_k) {
                end_i += 1;
            }
            begin_i = end_i;
            end_i += 1;

            curr_min = nums[begin_i];
            curr_max = curr_min;
            last_max_i = len;
            last_min_i = len;

            update(begin_i, &mut last_max_i, &mut last_min_i);

            update_ans(
                &mut ans,
                &begin_i,
                &curr_max,
                &curr_min,
                &last_max_i,
                &last_min_i,
            );
            // println!("{} {}", ans, end_i);
            continue;
        }
        curr_min = curr_min.min(curr);
        curr_max = curr_max.max(curr);
        update(end_i, &mut last_max_i, &mut last_min_i);
        println!("{}", ans);
        update_ans(
            &mut ans,
            &begin_i,
            &curr_max,
            &curr_min,
            &last_max_i,
            &last_min_i,
        );
        println!(
            "{} {} {} | {} {}",
            ans, begin_i, end_i, last_min_i, last_max_i
        );
        end_i += 1;
    }
    ans
}
