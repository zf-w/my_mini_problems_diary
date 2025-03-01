//! ## Leetcode 874. Walking Robot Simulation
//! https://leetcode.com/problems/walking-robot-simulation/
//! - `Medium`; `Independently Solved`; `2024-09-04`;

use std::collections::HashSet;

pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let mut obs_set: HashSet<(i32, i32)> = HashSet::with_capacity(obstacles.len());
    let mut curr_x: i32 = 0;
    let mut curr_y: i32 = 0;

    for obstacle in obstacles {
        obs_set.insert((obstacle[0], obstacle[1]));
    }

    let mut curr_direction: u8 = 0;
    let mut ans_dis = 0;
    const DIRECTION_LEN: u8 = 4;
    for command in commands {
        if command == -2 {
            curr_direction = (curr_direction + curr_direction - 1) % DIRECTION_LEN;
        } else if command == -1 {
            curr_direction = (curr_direction + 1) % DIRECTION_LEN;
        } else {
            let (mv_x, mv_y) = if curr_direction == 0 {
                (0, 1)
            } else if curr_direction == 1 {
                (1, 0)
            } else if curr_direction == 2 {
                (0, -1)
            } else if curr_direction == 3 {
                (-1, 0)
            } else {
                unreachable!()
            };
            for _ in 0..command {
                let next_pos = (curr_x + mv_x, curr_y + mv_y);
                if !obs_set.contains(&next_pos) {
                    (curr_x, curr_y) = next_pos;
                    ans_dis = ans_dis.max(curr_x * curr_x + curr_y * curr_y);
                } else {
                    break;
                }
            }
        }
    }
    ans_dis
}
