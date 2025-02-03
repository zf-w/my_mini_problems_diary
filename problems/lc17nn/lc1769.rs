//! https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/

pub fn min_operations(boxes: String) -> Vec<i32> {
    let str_bytes_ref = boxes.as_bytes();
    let len = str_bytes_ref.len();
    let flag_iter = str_bytes_ref
        .iter()
        .map(|v| -> bool { *v == b'1' }) // was 1 here. haha.
        .enumerate();
    let mut left_sum: usize = 0;
    let mut left_count: usize = 0;
    let mut right_sum: usize = 0;
    let mut right_count: usize = 0;

    for (i, contains_ball_flag) in flag_iter.clone() {
        if contains_ball_flag == true {
            right_sum += i;
            right_count += 1;
        }
    }

    let mut ans_vec = Vec::with_capacity(len);

    for (_, contains_ball_flag) in flag_iter.clone() {
        ans_vec.push((left_sum + right_sum) as i32);
        if contains_ball_flag == true {
            left_count += 1;
            right_count -= 1;
        }
        right_sum -= right_count;
        left_sum += left_count;
    }

    ans_vec
}
