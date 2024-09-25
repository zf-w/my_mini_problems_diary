//! ## Leetcode 2751. Robot Collisions
//! https://leetcode.com/problems/robot-collisions/
//! - `Hard`; `Hinted`; `2024-07-13`;

const CHAR_R_U8: u8 = 'R' as u8;

pub fn survived_robots_healths(
    positions: Vec<i32>,
    mut healths: Vec<i32>,
    directions: String,
) -> Vec<i32> {
    let len = positions.len();
    let mut idxs: Vec<usize> = Vec::with_capacity(len);
    let directions_bytes = directions.as_bytes();
    for i in 0..len {
        idxs.push(i);
    }
    idxs.sort_by_key(|v| -> i32 { unsafe { positions.get_unchecked(v.clone()).clone() } });
    let mut idx_stk: Vec<usize> = Vec::with_capacity(len);
    fn get_info(i: usize, healths: &Vec<i32>, directions: &[u8]) -> (i32, bool) {
        (unsafe { healths.get_unchecked(i).clone() }, unsafe {
            (*directions.get_unchecked(i)) == CHAR_R_U8
        })
    }
    for i in idxs.iter().cloned() {
        let (mut curr_health, is_curr_right) = get_info(i, &healths, directions_bytes);
        let mut push_curr_flag = true;
        if !is_curr_right {
            while let Some(last_i) = idx_stk.last().cloned() {
                let (prev_health, is_prev_right) =
                    get_info(last_i.clone(), &healths, directions_bytes);
                if is_prev_right {
                    if prev_health == curr_health {
                        idx_stk.pop();
                        push_curr_flag = false;
                        break;
                    } else if prev_health < curr_health {
                        unsafe { *healths.get_unchecked_mut(i) -= 1 };
                        curr_health -= 1;
                        idx_stk.pop();
                    } else {
                        unsafe { *healths.get_unchecked_mut(last_i) -= 1 };
                        push_curr_flag = false;
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        if push_curr_flag {
            idx_stk.push(i);
        }
    }
    let mut to_add_flags: Vec<bool> = vec![false; len];

    for i in idx_stk.iter().cloned() {
        unsafe { *to_add_flags.get_unchecked_mut(i) = true };
    }
    let mut ans_vec: Vec<i32> = Vec::with_capacity(len);

    for (health, need_add_flag) in healths.iter().cloned().zip(to_add_flags.iter().cloned()) {
        if need_add_flag {
            ans_vec.push(health);
        }
    }
    ans_vec
}
