//! # Leetcode 2327. Number of People Aware of a Secret
//! https://leetcode.com/problems/number-of-people-aware-of-a-secret/
//! - `Medium`; `y2025m09d09`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: dynamic_programming.

pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
    let n = n as usize;
    let delay = delay as usize;
    let forget = forget as usize;
    let mut change_arr: Box<[(i64, i64)]> = vec![(0, 0); n + 1].into_boxed_slice();

    let mut know_people_num: i64 = 1;
    let mut curr_people_num: i64 = 0;

    change_arr[delay].0 += 1;
    change_arr[forget].1 += 1;

    const MODOLUS: i64 = 1000000007;

    fn add_with_mod(num_mut_ref: &mut i64, add_num: i64) {
        *num_mut_ref = (*num_mut_ref + add_num) % MODOLUS;
    }

    fn sub_with_mod(num_mut_ref: &mut i64, sub_num: i64) {
        *num_mut_ref = (*num_mut_ref + MODOLUS - sub_num) % MODOLUS;
    }

    for day_i in 0..n {
        let new_people_num = change_arr[day_i].0;
        let forget_people_num = change_arr[day_i].1;

        add_with_mod(&mut curr_people_num, new_people_num);
        sub_with_mod(&mut curr_people_num, forget_people_num);

        add_with_mod(&mut know_people_num, curr_people_num);
        sub_with_mod(&mut know_people_num, forget_people_num);

        if day_i + delay <= n {
            add_with_mod(&mut change_arr[day_i + delay].0, curr_people_num);
        }

        if day_i + forget <= n {
            add_with_mod(&mut change_arr[day_i + forget].1, curr_people_num);
        }

        // println!("{} {}", curr_people_num, know_people_num);
    }

    know_people_num as i32
}
