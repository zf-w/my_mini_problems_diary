//! # Leetcode 3494. Find the Minimum Amount of Time to Brew Potions
//! https://leetcode.com/problems/find-the-minimum-amount-of-time-to-brew-potions/
//! - `Medium`; `y2025m10d09`; `Independently Solved`; `91ms`; `2.4mb`; `2 attempts`;
//! Topics: dynamic_programming.

pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
    let worker_num = skill.len();

    fn i32_to_i64(num: i32) -> i64 {
        num as i64
    }

    let mut ans_time: i64 = 0;

    let mut mana_iter = mana.into_iter().map(i32_to_i64);

    let mut time_vec: Vec<i64> = vec![0; worker_num];

    let first_mana = mana_iter.next().expect("mana.len() > 0");

    for (worker_i, worker_skill) in skill.iter().cloned().map(i32_to_i64).enumerate() {
        let work_time = worker_skill * first_mana;
        ans_time += work_time;
        time_vec[worker_i] = ans_time; // Be careful with the one potion case haha.
    }

    for mana in mana_iter {
        // print!("{} -> ", *time_vec.last().unwrap());
        for worker_i in (0..(worker_num - 1)).rev() {
            let work_time = (skill[worker_i] as i64) * mana;

            let best_begin_time = time_vec[worker_i + 1] - work_time;

            let time_entry_mut_ref = &mut time_vec[worker_i];

            (*time_entry_mut_ref) = (*time_entry_mut_ref).max(best_begin_time);
            // print!("{} -> ", *time_entry_mut_ref);
        }
        // println!("");

        time_vec[0] += (skill[0] as i64) * mana;

        for (worker_i, skill) in skill.iter().cloned().map(i32_to_i64).enumerate().skip(1) {
            let work_time = skill * mana;
            time_vec[worker_i] = time_vec[worker_i - 1] + work_time;
            // print!("{} -> ", time_vec[worker_i]);
        }
        // println!("\n");
        ans_time = *time_vec.last().expect("len > 0");
    }

    ans_time
}
