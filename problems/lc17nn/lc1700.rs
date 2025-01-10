//! ## Leetcode 1700. Number of Students Unable to Eat Lunch
//! https://leetcode.com/problems/number-of-students-unable-to-eat-lunch
//! - `Easy`; `Independently Solved`; `2024-04-07`;
//!
//! The key would be when the rest of the queue only consists of one type of student and the top sandwich doesn't match with their preference.

pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut prefer_0_num: usize = 0;
    let mut prefer_1_num: usize = 0;
    for student in students {
        match student {
            0 => prefer_0_num += 1,
            1 => prefer_1_num += 1,
            _ => (),
        }
    }

    for sandwich in sandwiches {
        match sandwich {
            0 => {
                if prefer_0_num > 0 {
                    prefer_0_num -= 1;
                } else {
                    return prefer_1_num as i32;
                }
            }
            1 => {
                if prefer_1_num > 0 {
                    prefer_1_num -= 1;
                } else {
                    return prefer_0_num as i32;
                }
            }
            _ => (),
        }
    }
    0
}
