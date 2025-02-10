//! ## Leetcode 3080. Mark Elements on Array by Performing Queries
//! https://leetcode.com/problems/mark-elements-on-array-by-performing-queries
//! - `Medium`; `Independently Solved`; `2024-03-16`;
//!
//!  This question is the second question of Leetcode Biweekly Contest 126.
//!
//! I guess a priority queue would also work. At first glance, I thought this question is about removing random elements from priority queue.

pub fn unmarked_sum_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let nums_len = nums.len();
    let mut sorted: Vec<(i32, usize)> = Vec::with_capacity(nums_len);
    let mut sum: i64 = 0;
    for (i, num) in nums.iter().enumerate() {
        sum += *num as i64;
        sorted.push((num.clone(), i));
    }
    sorted.sort();
    let mut nums_to_sorted: Vec<usize> = vec![0; nums_len];
    for (i1, (_, i0)) in sorted.iter().enumerate() {
        nums_to_sorted[*i0] = i1;
    }
    let mut marked = vec![false; nums_len];
    let mut lowest_unmarked: usize = 0;
    let q_len = queries.len();
    let mut ans: Vec<i64> = Vec::with_capacity(q_len);
    for query in queries {
        let index = query[0] as usize;
        let k = query[1] as usize;
        // println!("{} {} -", index, k);
        if marked[index] == false {
            sum -= nums[index] as i64;
            marked[index] = true;
        }
        let mut i = 0;
        while i < k {
            let to_mark = lowest_unmarked + i;
            if to_mark >= nums_len {
                break;
            }
            let map_to = sorted[to_mark].1;

            if marked[map_to] == true {
                lowest_unmarked += 1;
                continue;
            }
            i += 1;
            marked[map_to] = true;
            sum -= sorted[to_mark].0 as i64;
            // println!(" {}", sorted[to_mark].0);
        }
        lowest_unmarked += k;
        ans.push(sum);
    }
    ans
}

#[test]
fn q2_case_1() {
    let nums = vec![1, 2, 2, 1, 2, 3, 1];
    let qs = vec![vec![1, 2], vec![3, 3], vec![4, 2]];
    let expected = vec![8, 3, 0];
    assert_eq!(expected, unmarked_sum_array(nums, qs));
}

#[test]
fn q2_case_2() {
    let nums = vec![1, 4, 3, 2];
    let qs = vec![vec![0, 1]];
    let expected = vec![7];
    assert_eq!(expected, unmarked_sum_array(nums, qs));
}
