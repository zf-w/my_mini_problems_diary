//! ## Leetcode 786. K-th Smallest Prime Fraction
//! https://leetcode.com/problems/k-th-smallest-prime-fraction
//! - `Medium`; `Independently Solved`; `2024-05-11`;
//!
//!  This problem is about the k-th smallest fraction, but, since we know the array is sorted, the fractions of each numerator decrease from first to last. We can design an O(klog(n)) solution based on that.

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

pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::with_capacity(2);
    let len = arr.len();
    let last = len - 1;
    let mut pq: BinaryHeap<(Fraction, usize, usize)> = BinaryHeap::with_capacity(last);

    for i in 0..last {
        pq.push((Fraction::new(arr[i], arr[last]), i, last));
    }
    for _ in 0..(k - 1) {
        let (_, curr_n_i, curr_m_i) = pq.pop().expect("Should have a valid k...");
        let next_m_i = curr_m_i - 1;
        if next_m_i > curr_n_i {
            pq.push((
                Fraction::new(arr[curr_n_i], arr[next_m_i]),
                curr_n_i,
                next_m_i,
            ));
        }
    }
    let ans_frac = &pq.peek().expect("Should have a valid k").0;
    ans[0] = ans_frac.n;
    ans[1] = ans_frac.m;
    ans
}
