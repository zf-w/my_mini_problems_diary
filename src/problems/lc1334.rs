//! ## Leetcode 1334. Find the City With the Smallest Number of Neighbors at a Threshold Distance
//! https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/
//! - `Medium`; `Independently Solved`; `2024-07-26`;

pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    let n: usize = n as usize;
    let mut dis_vec: Vec<i32> = vec![i32::MAX; n * n];
    let index_fn = |node_0_i: usize, node_1_i: usize| -> usize { node_0_i * n + node_1_i };
    for edge_ref in edges.iter() {
        let node_0_i = edge_ref[0] as usize;
        let node_1_i = edge_ref[1] as usize;
        let weight = edge_ref[2];
        if weight <= distance_threshold {
            dis_vec[index_fn(node_0_i, node_1_i)] = weight;
            dis_vec[index_fn(node_1_i, node_0_i)] = weight;
        }
    }
    for node_i in 0..n {
        dis_vec[index_fn(node_i, node_i)] = 0;
    }
    for middle_i in 0..n {
        for node_0_i in 0..n {
            for node_1_i in 0..n {
                if node_0_i == node_1_i || middle_i == node_0_i || middle_i == node_1_i {
                    continue;
                }
                let curr_entry_i = index_fn(node_0_i, node_1_i);
                let part_0_dis = dis_vec[index_fn(node_0_i, middle_i)];
                let part_1_dis = dis_vec[index_fn(middle_i, node_1_i)];
                if part_0_dis == i32::MAX || part_1_dis == i32::MAX {
                    continue;
                }

                let curr_dis = part_0_dis + part_1_dis;
                if curr_dis < dis_vec[curr_entry_i] {
                    dis_vec[curr_entry_i] = curr_dis;
                }
            }
        }
    }
    let mut curr_min_info: Option<(usize, usize)> = None;
    for node_0_i in 0..n {
        let mut curr_count: usize = 0;
        for node_1_i in 0..n {
            if dis_vec[index_fn(node_0_i, node_1_i)] <= distance_threshold {
                curr_count += 1;
            }
        }
        // println!("{}:{}", node_0_i, curr_count);
        if let Some((curr_min_i, curr_min_count)) = curr_min_info.as_mut() {
            if curr_count <= *curr_min_count {
                *curr_min_i = node_0_i;
                *curr_min_count = curr_count;
            }
        } else {
            curr_min_info = Some((node_0_i, curr_count));
        }
    }
    if let Some((curr_min_i, _)) = curr_min_info {
        curr_min_i as i32
    } else {
        unreachable!()
    }
}
