//! ## Leetcode 1642. Furthest Building You Can Reach
//! https://leetcode.com/problems/furthest-building-you-can-reach
//! - `Medium`; `Independently Solved`; `2024-02-17`;
//!
//! An interesting problem. TLE at first with brute-force recursion.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn solve(problems: &[i32], mut bricks: i32, mut ladders: i32) -> usize {
    let len = problems.len();

    let mut hp: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(len);

    // let mut overcome: usize = 0;
    let mut i: usize = 0;
    while i < len && (bricks > 0 || ladders > 0) {
        let need = problems[i];
        if ladders > 0 {
            hp.push(Reverse(need));
            i += 1;
            ladders -= 1;
        } else {
            if let Some(Reverse(prev)) = hp.pop() {
                let mn = prev.min(need);
                let mx = prev.max(need);
                if bricks >= mn {
                    bricks -= mn;
                    i += 1;
                    hp.push(Reverse(mx));
                } else {
                    break;
                }
            } else {
                if bricks >= need {
                    bricks -= need;
                    i += 1;
                } else {
                    break;
                }
            }
        }
    }
    i
}

pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
    let len = heights.len();
    let mut problems: Vec<i32> = Vec::with_capacity(len - 1);
    let mut milestones: Vec<usize> = Vec::with_capacity(len - 1);
    let mut iter = heights.iter();
    let mut prev: &i32 = iter.next().expect("Should have at least one element");

    for (i, curr) in iter.enumerate() {
        if *curr > *prev {
            problems.push(curr - prev);
            milestones.push(i + 1);
            // println!("{}, {}", i + 1, curr - prev);
        }
        prev = curr;
    }

    let overcome = solve(&problems, bricks, ladders);
    // println!("{}", overcome);
    (if overcome == milestones.len() {
        len - 1
    } else if overcome == 0 {
        milestones[0] - 1
    } else {
        milestones[overcome] - 1
    }) as i32
}
