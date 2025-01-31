//! # Leetcode 827. Making A Large Island
//! https://leetcode.com/problems/making-a-large-island/
//! - `Hard`; `y2025m01d31`; `Independently Solved`; `38ms`; `7.7mb`; `1 attempt`;

#[derive(Clone)]
enum SizeOrIdx {
    Size(usize),
    Idx(usize),
}

struct UnionFind {
    info_vec: Vec<SizeOrIdx>,
    pub max_group_size: usize,
}

impl UnionFind {
    pub fn new_with_size(size: usize) -> Self {
        Self {
            info_vec: vec![SizeOrIdx::Size(1); size],
            max_group_size: 1,
        }
    }

    pub fn find(&mut self, query_node_i: usize) -> (usize, usize) {
        let curr_info = self.info_vec[query_node_i].clone();
        let parent_idx = match curr_info {
            SizeOrIdx::Size(group_size) => {
                return (query_node_i, group_size);
            }
            SizeOrIdx::Idx(parent_idx) => parent_idx,
        };
        let (root_group_idx, root_group_size) = self.find(parent_idx);
        self.info_vec[query_node_i] = SizeOrIdx::Idx(root_group_idx);
        (root_group_idx, root_group_size)
    }

    pub fn union(&mut self, node_0_i: usize, node_1_i: usize) -> bool {
        let (group_0_i, group_0_size) = self.find(node_0_i);
        let (group_1_i, group_1_size) = self.find(node_1_i);

        if group_0_i == group_1_i {
            return false;
        }
        let size_sum = group_0_size + group_1_size;
        self.info_vec[group_1_i] = SizeOrIdx::Idx(group_0_i);
        self.info_vec[group_0_i] = SizeOrIdx::Size(size_sum);
        self.max_group_size = self.max_group_size.max(size_sum);
        true
    }
}

type NeighbourOffFn = fn(usize, usize, usize, usize) -> Option<(usize, usize)>;

const NEIGHBOUR_OFF_FN_ARR: [NeighbourOffFn; 4] = [
    |row_i, col_i, _, _| {
        if row_i > 0 {
            Some((row_i - 1, col_i))
        } else {
            None
        }
    },
    |row_i, col_i, row_len, _| {
        if row_i + 1 < row_len {
            Some((row_i + 1, col_i))
        } else {
            None
        }
    },
    |row_i, col_i, _, _| {
        if col_i > 0 {
            Some((row_i, col_i - 1))
        } else {
            None
        }
    },
    |row_i, col_i, _, col_len| {
        if col_i + 1 < col_len {
            Some((row_i, col_i + 1))
        } else {
            None
        }
    },
];

pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut union_find = UnionFind::new_with_size(len * len);
    let fn_calc_idx = |row_i: usize, col_i: usize| -> usize { row_i * len + col_i };

    for row_i in 1..len {
        if grid[row_i][0] != 1 {
            continue;
        }
        let curr_idx = fn_calc_idx(row_i, 0);
        if grid[row_i - 1][0] == 1 {
            union_find.union(curr_idx, fn_calc_idx(row_i - 1, 0));
        }
    }

    for col_i in 1..len {
        if grid[0][col_i] != 1 {
            continue;
        }
        let curr_idx = fn_calc_idx(0, col_i);
        if grid[0][col_i - 1] == 1 {
            union_find.union(curr_idx, fn_calc_idx(0, col_i - 1));
        }
    }

    for row_i in 1..len {
        for col_i in 1..len {
            if grid[row_i][col_i] != 1 {
                continue;
            }
            let curr_idx = fn_calc_idx(row_i, col_i);
            if grid[row_i - 1][col_i] == 1 {
                union_find.union(curr_idx, fn_calc_idx(row_i - 1, col_i));
            }
            if grid[row_i][col_i - 1] == 1 {
                union_find.union(curr_idx, fn_calc_idx(row_i, col_i - 1));
            }
        }
    }

    let mut ans_size = union_find.max_group_size;

    let mut group_info_vec: Vec<usize> = Vec::with_capacity(4);

    for row_i in 0..len {
        for col_i in 0..len {
            if grid[row_i][col_i] == 1 {
                continue;
            }

            let mut curr_size = 1;

            for off_fn in NEIGHBOUR_OFF_FN_ARR.iter() {
                let (next_row_i, next_col_i) = if let Some(res) = off_fn(row_i, col_i, len, len) {
                    res
                } else {
                    continue;
                };

                if grid[next_row_i][next_col_i] == 0 {
                    continue;
                }
                let (g_i, g_size) = union_find.find(fn_calc_idx(next_row_i, next_col_i));
                if group_info_vec.contains(&g_i) == false {
                    group_info_vec.push(g_i);
                    curr_size += g_size;
                }
            }

            group_info_vec.clear();
            ans_size = ans_size.max(curr_size);
        }
    }

    ans_size as i32
}
