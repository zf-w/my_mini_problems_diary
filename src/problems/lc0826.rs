//! ## Leetcode 826. Most Profit Assigning Work
//! https://leetcode.com/problems/most-profit-assigning-work/
//! - `Medium`; `Independently Solved`; `2024-06-18`;
//!
//! We can sort the profits based on difficulties and workers. Then, we can easily assign each worker their most profitable job by looping through jobs from low difficulty to high difficulty and keeping track of the most profitable job.

pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, mut workers: Vec<i32>) -> i32 {
    let job_len = difficulty.len();
    let mut idxs: Vec<usize> = Vec::with_capacity(job_len);
    for i in 0..job_len {
        idxs.push(i);
    }
    idxs.sort_by_key(|i| -> (i32, i32) { (difficulty[*i], profit[*i]) });
    let mut curr_max_profit: i32 = 0;
    let mut job_i: usize = 0;
    let mut ans_profit = 0;
    workers.sort();
    for worker in workers.iter() {
        while job_i < job_len && difficulty[idxs[job_i]] <= *worker {
            curr_max_profit = curr_max_profit.max(profit[job_i]);
            job_i += 1;
        }
        ans_profit += curr_max_profit;
    }

    ans_profit
}
