//! # Leetcode 2127. Maximum Employees to Be Invited to a Meeting
//! https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/
//! - `Hard`; `y2025m01d26`; `Learned from Solution`; `7ms`; `5.02mb`; `1 attempt`;
//!
//! Learned from https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/editorial

pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
    use std::collections::VecDeque;
    let len = favorite.len();
    let mut degree_vec: Vec<usize> = vec![0; len];

    for favorite_i in favorite.iter().cloned().map(|v| -> usize { v as usize }) {
        degree_vec[favorite_i] += 1;
    }

    let mut wait_queue: VecDeque<usize> = VecDeque::with_capacity(len);

    for (i, curr_degree) in degree_vec.iter().cloned().enumerate() {
        if curr_degree == 0 {
            wait_queue.push_back(i);
        }
    }

    let mut depth_vec: Vec<usize> = vec![0; len];

    while wait_queue.is_empty() == false {
        let curr_i = wait_queue.pop_front().expect("checked len.");

        let next_i = favorite[curr_i] as usize;

        depth_vec[next_i] = depth_vec[next_i].max(depth_vec[curr_i] + 1);

        degree_vec[next_i] -= 1;

        if degree_vec[next_i] == 0 {
            wait_queue.push_back(next_i);
        }
    }

    let mut longest_cycle_len: usize = 0;
    let mut two_cycle_invitations_len: usize = 0;

    for i in 0..len {
        if degree_vec[i] == 0 {
            continue;
        }

        let mut cycle_len: usize = 0;
        let mut curr_i = i;

        while degree_vec[curr_i] != 0 {
            degree_vec[curr_i] = 0;
            cycle_len += 1;
            curr_i = favorite[curr_i] as usize;
        }

        if cycle_len == 2 {
            two_cycle_invitations_len += depth_vec[i] + depth_vec[favorite[i] as usize] + 2;
        } else {
            longest_cycle_len = longest_cycle_len.max(cycle_len);
        }
    }

    longest_cycle_len.max(two_cycle_invitations_len) as i32
}
