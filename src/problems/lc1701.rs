//! ## Leetcode 1701. Average Waiting Time
//! https://leetcode.com/problems/average-waiting-time/
//! - `Medium`;; `Independently Solved`; `2024-07-10`;

pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let mut waiting_time_sum: u64 = 0;
    let mut curr_time = 0;
    for customer in customers.iter() {
        let come_in_time = unsafe { customer.get_unchecked(0).clone() };
        let need_time = unsafe { customer.get_unchecked(1).clone() };

        if curr_time < come_in_time {
            curr_time = come_in_time;
        }
        curr_time += need_time;
        waiting_time_sum += (curr_time - come_in_time) as u64;
    }
    waiting_time_sum as f64 / customers.len() as f64
}
