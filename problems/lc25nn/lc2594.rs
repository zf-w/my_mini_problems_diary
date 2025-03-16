//! # Leetcode 2594. Minimum Time to Repair Cars
//! https://leetcode.com/problems/minimum-time-to-repair-cars/
//! - `Medium`; `y2025m03d16`; `Hinted`; `13ms`; `2.6mb`; `1 attempt`;
//! Topics: binary_search.
//!
//! I used the hint to verify whether this problem is approachable with `binary_search`.

pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    let mut begin_time: i64 = 0;
    let mut end_time: i64 = i64::MAX;

    fn calc_num_cars_one_worker_can_fix_in_time(worker_rank: i32, time_limit: i64) -> i32 {
        let worker_rank = worker_rank as i64;
        let temp = time_limit / worker_rank;
        (temp as f64).sqrt() as i32
    }

    let tell_if_workers_can_finish_in_time = |time_limit: i64| -> bool {
        let mut fixed_cars: i32 = 0;
        for worker_rank_ref in ranks.iter() {
            fixed_cars += calc_num_cars_one_worker_can_fix_in_time(*worker_rank_ref, time_limit);
            if fixed_cars >= cars {
                return true;
            }
        }
        false
    };

    while begin_time < end_time {
        let mid_time_limit = (begin_time + end_time) / 2;

        if tell_if_workers_can_finish_in_time(mid_time_limit) == true {
            end_time = mid_time_limit;
        } else {
            begin_time = mid_time_limit + 1;
        }
    }

    begin_time
}
