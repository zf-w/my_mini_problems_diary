//! # Leetcode 762. Prime Number of Set Bits in Binary Representation
//! https://leetcode.com/problems/prime-number-of-set-bits-in-binary-representation/
//! - y2026m02d21; Independently Solved;

pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut ans_count = 0;

    for num in left..=right {
        let one_bit_count = num.count_ones();

        if [2, 3, 5, 7, 11, 13, 17, 19, 23, 31].contains(&(one_bit_count as i32)) {
            ans_count += 1;
        }
    }

    ans_count
}