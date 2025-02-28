//! ## Leetcode 857. Minimum Cost to Hire K Workers
//! https://leetcode.com/problems/minimum-cost-to-hire-k-workers
//! - `Hard`; `Learned from Solution`; `2024-05-11`;
//!
//! Read Solution: https://leetcode.com/problems/minimum-cost-to-hire-k-workers/solutions/5141780/easy-c-beats-100-with-explanation
//!
//! I guess one key to solving this problem is finding out that the minimum wage-to-quality ratio and the sum of the qualities determines the current total wages needed.

use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Fraction {
    n: i32,
    m: i32,
}

impl Fraction {
    pub fn new(n: i32, m: i32) -> Self {
        Fraction { n, m }
    }

    pub fn to_f64(&self) -> f64 {
        self.n as f64 / self.m as f64
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (other.n * &self.m).partial_cmp(&(self.n * other.m))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Should have")
    }
}

pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    let len = quality.len();
    let mut ratios: Vec<Fraction> = Vec::with_capacity(len);
    for (q, w) in quality.into_iter().zip(wage.into_iter()) {
        ratios.push(Fraction::new(w, q));
    }
    let mut pq: BinaryHeap<i32> = BinaryHeap::with_capacity(len);
    ratios.sort();
    let mut ratio_iter = ratios.iter();
    let mut count = 0;
    let mut ans: f64;
    let mut ratio: f64 = 0.0;
    let mut quality_sum: i32 = 0;
    while count < k {
        let f = ratio_iter.next().expect("Should have a valid k");
        ratio = f.to_f64();
        let curr_quality = f.m;
        quality_sum += curr_quality;
        pq.push(curr_quality);
        count += 1;
    }
    ans = ratio * quality_sum as f64;
    while let Some(f) = ratio_iter.next() {
        let pop_quality = pq.pop().expect("Should have something");
        quality_sum -= pop_quality;
        let curr_quality = f.m;
        quality_sum += curr_quality;
        pq.push(quality_sum);
        ans = ans.min(ratio * quality_sum as f64);
    }

    ans
}
