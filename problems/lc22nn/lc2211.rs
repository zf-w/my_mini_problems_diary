//! # Leetcode 2211. Count Collisions on a Road
//! https://leetcode.com/problems/count-collisions-on-a-road/
//! - `Medium`; `y2025m12d04`; `Independently Solved`; `7ms`; `2.4mb`; `2 attempts`;

pub fn count_collisions(directions: String) -> i32 {
    let mut left_blocked_flag: bool = false;
    let mut going_right_car_count: usize = 0;

    let mut ans_count = 0;

    for c in directions.chars() {
        if c == 'R' {
            going_right_car_count += 1;
        } else if c == 'L' {
            if going_right_car_count > 0 {
                ans_count += 1 + going_right_car_count as i32;
                going_right_car_count = 0;
                left_blocked_flag = true;
            } else if left_blocked_flag == true {
                ans_count += 1;
            }
        } else {
            if going_right_car_count > 0 {
                ans_count += going_right_car_count as i32;
                going_right_car_count = 0;
            }
            left_blocked_flag = true; // Stationary cars also need to block the road. 
        }
    }  

    ans_count      
}