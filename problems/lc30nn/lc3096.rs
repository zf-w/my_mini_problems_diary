//! ## Leetcode 3096. Minimum Levels to Gain More Points
//! https://leetcode.com/problems/minimum-levels-to-gain-more-points/
//! - `Medium`; `Independently Solved`; `2024-03-30`;
//!
//! The prefix sum array can help us to quickly calculate the score.

pub fn minimum_levels(possible: Vec<i32>) -> i32 {
    let len = possible.len();
    let mut sums: Vec<i32> = Vec::with_capacity(len);
    let mut curr = 0;
    for num in possible.iter() {
        if num == &0 {
            curr -= 1;
        } else {
            curr += 1;
        }
        sums.push(curr);
    }
    let total = sums[len - 1];
    for (i, sum) in sums.iter().enumerate() {
        let me = sum;
        let other = total - sum;
        if me > &other {
            if i + 1 == len {
                return -1;
            }
            return i as i32 + 1;
        }
    }

    return -1;
}

#[test]
fn check_case_0() {
    let possible = vec![1, 1, 1, 1, 1];
    assert_eq!(3, minimum_levels(possible));
}

#[test]
fn check_case_1() {
    let possible = vec![0, 0];
    assert_eq!(-1, minimum_levels(possible));
}

#[test]
fn check_case_2() {
    let possible = vec![1, 0, 1, 0];
    assert_eq!(1, minimum_levels(possible));
}

#[test]
fn check_case_3() {
    let possible = vec![1, 1];
    assert_eq!(-1, minimum_levels(possible));
}
