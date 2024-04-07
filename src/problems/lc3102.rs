//! ## Leetcode 3102. Minimize Manhattan Distances
//! https://leetcode.com/problems/minimize-manhattan-distances
//! - `Hard`; `TLE`; `2024-03-30`;
//!
//! A weird approach I tried to minimize the manhattan distances.

pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
    let len = points.len();
    let mut dists: Vec<(i32, usize, i32, usize)> = vec![(0, len, 0, len); len];

    let cal_dis = |a: &[i32], b: &[i32]| -> i32 { (a[0] - b[0]).abs() + (a[1] - b[1]).abs() };
    let update_dis = |entry: &mut (i32, usize, i32, usize), curr_dis: i32, other_i: usize| {
        if entry.1 == len {
            entry.0 = curr_dis;
            entry.1 = other_i;
        } else {
            if entry.0 < curr_dis {
                entry.2 = entry.0;
                entry.3 = entry.1;
                entry.0 = curr_dis;
                entry.1 = other_i;
            } else if entry.3 == len || entry.2 < curr_dis {
                entry.2 = curr_dis;
                entry.3 = other_i;
            }
        }
    };
    let mut max_dis = -1;
    let mut max_i = (len, len);
    for i in 0..2 {
        for j in (i + 1)..len {
            let curr = cal_dis(&points[i], &points[j]);
            if curr > max_dis {
                max_dis = curr;
                max_i = (i, j);
            }
            update_dis(&mut dists[i], curr, j);
            update_dis(&mut dists[j], curr, i);
        }
    }
    fn check_dists_ignore(dists: &[(i32, usize, i32, usize)], i: usize) -> i32 {
        let mut ans = -1;
        for (j, (a, a_i, b, b_i)) in dists.iter().enumerate() {
            if j == i {
                continue;
            }
            if a_i != &i {
                ans = ans.max(*a);
            }
            if b_i != &i {
                ans = ans.max(*b);
            }
        }
        ans
    }
    // println!("{:?}", dists);
    // println!("{:?}", max_i);
    let ans = check_dists_ignore(&dists, max_i.0);
    ans.min(check_dists_ignore(&dists, max_i.1))
}

#[test]
fn check_case_0() {
    let points = vec![vec![3, 10], vec![5, 15], vec![10, 2], vec![4, 4]];
    assert_eq!(12, minimum_distance(points));
}

#[test]
fn check_case_1() {
    let points = vec![vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 1]];
    assert_eq!(0, minimum_distance(points));
}
