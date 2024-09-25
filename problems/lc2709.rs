//! ## Leetcode 2709. Greatest Common Divisor
//! https://leetcode.com/problems/greatest-common-divisor-traversal
//! - `Hard`; `Hinted by Solution`; `2024-02-25`;
//!
//! A very comprehensive question about primes and graph concepts.

use crate::util::UnionFind;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a > 0 {
        let temp = b % a;
        b = a;
        a = temp;
    }
    b
}

fn primes_before(n: usize) -> Vec<usize> {
    let mut ans = Vec::with_capacity(n / (3 * n.ilog2()) as usize);
    let len = n + 1;
    let mut is_prime: Vec<bool> = vec![true; len];
    for i in 2..len {
        if !is_prime[i] {
            continue;
        }
        ans.push(i);
        let mut j = 2 * i;
        while j < len {
            is_prime[j] = false;
            j += i;
        }
    }
    ans
}

pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
    let len = nums.len();
    if len == 1 {
        return true;
    }
    let nums_max = *nums.iter().max().unwrap();
    let nums_len = nums_max as usize + 1;
    let mut st: Vec<bool> = vec![false; nums_len];

    for num in nums.iter() {
        let idx = *num as usize;
        if !st[idx] {
            st[idx] = true;
        }
    }
    if st[1] {
        return false;
    }
    let primes = primes_before(nums_max as usize);

    let mut uf = UnionFind::new_with_size(nums_len);

    for p in primes.iter() {
        let mut i = p + p;
        // println!("{}", p);
        while i < nums_len {
            if st[i as usize] {
                // println!("{} {}", *p, i);
                uf.union(*p as usize, i as usize);
            }
            i += p;
        }
    }

    let mut nums_iter = nums.iter();
    let dest: usize = uf.find(*nums_iter.next().expect("Should have an element") as usize);
    for num in nums_iter {
        if dest != uf.find(*num as usize) {
            return false;
        }
    }
    true
}
#[test]
fn check_case1() {
    assert_eq!(can_traverse_all_pairs(vec![2, 3, 6]), true)
}
