//! ## Leetcode 2285. Maximum Total Importance of Roads
//! https://leetcode.com/problems/maximum-total-importance-of-roads/
//! - `Medium`; `Independently Solved`; `2024-06-27`;
//!
//! We can solve this problem by assigning nodes having larger degrees more importance scores: sorting the node degrees and calculating the sum of the scores multiplied by the nodes' degrees.

pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut counts: Vec<usize> = vec![0; n];
    let mut idxs: Vec<usize> = Vec::with_capacity(n);
    for road in roads.iter() {
        let node_0_i = road[0] as usize;
        let node_1_i = road[1] as usize;
        counts[node_0_i] += 1;
        counts[node_1_i] += 1;
    }
    for i in 0..n {
        idxs.push(i);
    }
    let mut ans_sum: usize = 0;
    idxs.sort_unstable_by_key(|i| -> &usize { &counts[*i] });
    for (i, sorted_i) in idxs.iter().enumerate() {
        ans_sum += i * counts[*sorted_i];
    }
    // for (i, sorted_i) in idxs.iter().enumerate() {
    //     counts[*sorted_i] = n - i;
    // }
    //
    // for road in roads.iter() {
    //     let node_0_i = road[0] as usize;
    //     let node_1_i = road[1] as usize;
    //     ans_sum += counts[node_0_i] + counts[node_1_i];
    // }
    ans_sum as i64
}
