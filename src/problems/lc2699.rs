use std::{cmp::Reverse, collections::BinaryHeap};

fn run_dijkstra(
    adj_vec_ref: &Vec<Vec<(usize, usize)>>,
    edges: &mut Vec<Vec<i32>>,
    dists_mut_ref: &mut Vec<[i32; 2]>,
    source: usize,
    difference: i32,
    run: usize,
) {
    let mut pq = BinaryHeap::with_capacity(adj_vec_ref.len());
    pq.push(Reverse((0, source)));
    dists_mut_ref[source][run] = 0;

    while let Some(Reverse((current_distance, current_node))) = pq.pop() {
        if current_distance > dists_mut_ref[current_node][run] {
            continue;
        }

        for &(next_node, edge_index) in &adj_vec_ref[current_node] {
            let mut weight = edges[edge_index][2];
            if weight == -1 {
                weight = 1;
            }

            if run == 1 && edges[edge_index][2] == -1 {
                let new_weight =
                    difference + dists_mut_ref[next_node][0] - dists_mut_ref[current_node][1];
                if new_weight > weight {
                    edges[edge_index][2] = new_weight;
                    weight = new_weight;
                }
            }

            let new_distance = dists_mut_ref[current_node][run].saturating_add(weight);
            if new_distance < dists_mut_ref[next_node][run] {
                dists_mut_ref[next_node][run] = new_distance;
                pq.push(Reverse((new_distance, next_node)));
            }
        }
    }
}

pub fn modified_graph_edges(
    n: i32,
    mut edges: Vec<Vec<i32>>,
    source: i32,
    destination: i32,
    target: i32,
) -> Vec<Vec<i32>> {
    let nodes_len = n as usize;
    let from_node_i = source as usize;
    let to_node_i = destination as usize;
    let mut adj_vec: Vec<Vec<(usize, usize)>> = vec![Vec::new(); nodes_len];
    for (edge_i, edge_ref) in edges.iter().enumerate() {
        let node_a_i = edge_ref[0] as usize;
        let node_b_i = edge_ref[1] as usize;
        adj_vec[node_a_i].push((node_b_i, edge_i));
        adj_vec[node_b_i].push((node_a_i, edge_i));
    }

    let mut dists: Vec<[i32; 2]> = vec![[i32::MAX; 2]; nodes_len];
    run_dijkstra(&adj_vec, &mut edges, &mut dists, from_node_i, 0, 0);
    let diff = target - dists[to_node_i][0];
    if diff < 0 {
        return Vec::new();
    }

    run_dijkstra(&adj_vec, &mut edges, &mut dists, from_node_i, diff, 1);
    if dists[to_node_i][1] < target {
        return Vec::new();
    }

    for edge_vec_mut_ref in edges.iter_mut() {
        if edge_vec_mut_ref[2] == -1 {
            edge_vec_mut_ref[2] = 1;
        }
    }

    edges
}
