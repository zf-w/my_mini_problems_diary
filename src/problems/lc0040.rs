//! ## Leetcode 40. Combination Sum II
//! https://leetcode.com/problems/combination-sum-ii/
//! - `Medium`; `Independently Solved`; `2024-08-13`;

use std::{
    collections::HashSet,
    hash::{DefaultHasher, Hash, Hasher},
};

fn hash_fn(path_vec_ref: &Vec<i32>) -> u64 {
    let mut hasher = DefaultHasher::new();
    path_vec_ref.hash(&mut hasher);
    hasher.finish()
}

fn helper_1(
    candidates_arr_ref: &[i32],
    path_vec_mut_ref: &mut Vec<i32>,
    path_sum_mut_ref: &mut i32,
    ans_vec_mut_ref: &mut Vec<Vec<i32>>,
    hash_set: &mut HashSet<u64>,
    target: i32,
    mut remaining_total_sum: i32,
) {
    for (i, curr_num) in candidates_arr_ref.iter().cloned().enumerate() {
        let curr_sum = *path_sum_mut_ref + curr_num;
        remaining_total_sum -= curr_num;
        if curr_sum == target {
            path_vec_mut_ref.push(curr_num);
            *path_sum_mut_ref += curr_num;
            let curr_path_id = hash_fn(&path_vec_mut_ref);
            if !hash_set.contains(&curr_path_id) {
                ans_vec_mut_ref.push(path_vec_mut_ref.clone());
                hash_set.insert(curr_path_id);
            }

            path_vec_mut_ref.pop();
            *path_sum_mut_ref -= curr_num;
        } else if curr_sum < target && i + 1 < candidates_arr_ref.len() {
            if remaining_total_sum < target - curr_sum {
                break;
            }
            path_vec_mut_ref.push(curr_num);
            *path_sum_mut_ref += curr_num;
            helper_1(
                &candidates_arr_ref[(i + 1)..],
                path_vec_mut_ref,
                path_sum_mut_ref,
                ans_vec_mut_ref,
                hash_set,
                target,
                remaining_total_sum,
            );
            path_vec_mut_ref.pop();
            *path_sum_mut_ref -= curr_num;
        } else {
            break;
        }
    }
}

pub fn combination_sum2_1(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ans_vec: Vec<Vec<i32>> = Vec::new();
    candidates.sort_unstable();
    let mut path_vec: Vec<i32> = Vec::with_capacity(candidates.len());
    let mut hash_set: HashSet<u64> = HashSet::new();
    let mut path_sum: i32 = 0;
    let remaining_total_sum: i32 = candidates.iter().sum();
    helper_1(
        &candidates,
        &mut path_vec,
        &mut path_sum,
        &mut ans_vec,
        &mut hash_set,
        target,
        remaining_total_sum.clone(),
    );
    ans_vec
}

fn make_count_vec_to_ans_vec(path_vec_ref: &Vec<(i32, usize)>) -> Vec<i32> {
    let mut ans_vec: Vec<i32> = Vec::with_capacity(100);
    for (curr_num, curr_count) in path_vec_ref.iter().cloned() {
        for _ in 0..curr_count {
            ans_vec.push(curr_num);
        }
    }
    ans_vec
}

fn helper(
    counts_ref: &[usize; 51],
    i: usize,
    mut curr_sum: i32,
    mut remaining_sum: i32,
    ans_vec_mut_ref: &mut Vec<Vec<i32>>,
    path_vec_mut_ref: &mut Vec<(i32, usize)>,
    target: i32,
) {
    let curr_num = i as i32;
    for count in 0..=(counts_ref[i]) {
        if curr_sum + remaining_sum < target {
            return;
        }
        path_vec_mut_ref.push((curr_num, count));
        if curr_sum == target {
            ans_vec_mut_ref.push(make_count_vec_to_ans_vec(&path_vec_mut_ref));
        } else if curr_sum < target && i < 50 {
            helper(
                counts_ref,
                i + 1,
                curr_sum,
                remaining_sum,
                ans_vec_mut_ref,
                path_vec_mut_ref,
                target,
            );
        } else {
            path_vec_mut_ref.pop();
            break;
        }
        path_vec_mut_ref.pop();
        curr_sum += curr_num;
        remaining_sum -= curr_num;
    }
}

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ans_vec: Vec<Vec<i32>> = Vec::new();
    let mut counts: [usize; 51] = [0; 51];
    let mut remaining_sum = 0;
    for candidate in candidates {
        counts[candidate as usize] += 1;
        remaining_sum += candidate;
    }
    let mut path_vec: Vec<(i32, usize)> = Vec::with_capacity(50);
    helper(
        &counts,
        1,
        0,
        remaining_sum,
        &mut ans_vec,
        &mut path_vec,
        target,
    );
    ans_vec
}
