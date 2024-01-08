//! ## Leetcode 1235. Maximum Profit in Job Scheduling
//! https://leetcode.com/problems/maximum-profit-in-job-scheduling
//! - `Hard`; `Learned from previous`; `2024-01-05`;
//! 
//! It's good to know the derived codes implementing those traits like "Ord" follow the order of the definition of the struct.

use std::collections::BTreeMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Job {
  start: i32,
  end: i32,
  profit: i32
}

impl Job {
  pub fn new(start: i32, end: i32, profit: i32) -> Self {
    Job {
      start,
      end,
      profit
    }
  }
}

pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
  let len = start_time.len();
  let mut jobs: Vec<Job> = Vec::with_capacity(len);
  
  for i in 0..len {
    jobs.push(Job::new(start_time[i], end_time[i], profit[i]));
  }

  jobs.sort();

  let mut m: BTreeMap<i32, i32> = BTreeMap::new();
  m.insert(0, 0);
  for job in jobs.iter() {
    let curr = m.range(..job.start).last().unwrap().1 + job.profit;
    if curr > *m.last_key_value().unwrap().1 {
      if let Some(v) = m.get_mut(&job.end) {
        *v = curr;
      } else {
        m.insert(job.end, curr);
      }
    }
  }
  *m.last_key_value().unwrap().1
}