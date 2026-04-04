//! # Leetcode 3296. Minium Number of Seconds to Make Mountain Height Zero
//! https://leetcode.com/problems/minimum-number-of-seconds-to-make-mountain-height-zero/
//! - y2026m03d13; Independently Solved;

pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
    use std::collections::HashMap;

    fn calc_height_reduced_within_time(worker_time: i32, time: i64) -> i64 {
        let unit_time = 2 * time / (worker_time as i64);

        ((-1.0 + (f64::sqrt((1 + 4 * unit_time) as f64))) / 2.0).floor() as i64
    }

    let mut worker_count_map: HashMap<i32, usize> = HashMap::new();

    for worker_time in worker_times {
        worker_count_map
            .entry(worker_time)
            .and_modify(|count_mut_ref| *count_mut_ref += 1)
            .or_insert(1);
    }

    let worker_time_and_count_vec: Vec<(i32, usize)> = worker_count_map.into_iter().collect();

    let closure_test_if_enough_time = |time: i64| -> bool {
        let mut height_reduced_within_time: i64 = 0;

        for (worker_time, count) in worker_time_and_count_vec.iter().cloned() {
            height_reduced_within_time +=
                (count as i64) * calc_height_reduced_within_time(worker_time, time);
            // println!("{} {} {}",worker_time, time, calc_height_reduced_within_time(worker_time, time));
        }
        // println!("{} {}", height_reduced_within_time, mountain_height);
        height_reduced_within_time >= (mountain_height as i64)
    };

    let mut time_lower_bound: i64 = 0;
    let mut time_upper_exc_bound: i64 = (1i64 << 54) as i64;

    while time_lower_bound < time_upper_exc_bound {
        let time_to_test = (time_upper_exc_bound + time_lower_bound) / 2;
        // println!("{} {}", time_lower_bound, time_upper_exc_bound);

        if closure_test_if_enough_time(time_to_test) {
            time_upper_exc_bound = time_to_test;
        } else {
            time_lower_bound = time_to_test + 1;
        }
    }

    time_lower_bound
}