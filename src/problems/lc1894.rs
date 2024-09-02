//! ## Leetcode 1894. Find the Student that Will Replace the Chalk
//! https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk/
//! - `Medium`; `Independently Solved`; `2024-09-02`;

type Num = u64;

pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
    let mut k = k as Num;
    let mut prev_chalk_sum: Num = 0;
    let len = chalk.len();
    let mut chalk_pre_sum_vec: Vec<Num> = Vec::with_capacity(len);
    for curr_ref in chalk.iter() {
        prev_chalk_sum += *curr_ref as Num;
        chalk_pre_sum_vec.push(prev_chalk_sum);
    }
    k = k % prev_chalk_sum;
    // println!("{}",k);
    let mut begin_i = 0;
    let mut end_i = chalk.len();
    while begin_i < end_i {
        let mid_i = (begin_i + end_i) / 2;
        let mid_v = chalk_pre_sum_vec[mid_i];
        if mid_v == k {
            return (mid_i + 1) as i32;
        } else if mid_v < k {
            begin_i = mid_i + 1;
        } else {
            end_i = mid_i;
        }
    }
    begin_i as i32
}
