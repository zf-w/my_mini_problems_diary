//! Leetcode 1352. Product of the Last K Numbers
//! https://leetcode.com/problems/product-of-the-last-k-numbers/
//! - `Medium`; `y2025m02d13`; `Independently Solved`; `22ms`; `19.1mb`; `2 attempts`;

struct ProductOfNumbers {
    prefix_product_vec: Vec<(i32, i32)>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        let mut prefix_product_vec: Vec<(i32, i32)> = Vec::with_capacity(10);
        prefix_product_vec.push((1, 0));
        Self { prefix_product_vec }
    }

    fn add(&mut self, num: i32) {
        let last_prefix_prod_info = self
            .prefix_product_vec
            .last()
            .cloned()
            .expect("should be valid.");
        if num != 0 {
            self.prefix_product_vec
                .push((last_prefix_prod_info.0 * num, last_prefix_prod_info.1));
        } else {
            self.prefix_product_vec
                .push((1, last_prefix_prod_info.1 + 1)); // Be careful with product overflow, was "last_prefix_prod_info.0" before. Haha.
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let to_divid_idx = self.prefix_product_vec.len() - 1 - k as usize;
        let last_prefix_prod_info = self
            .prefix_product_vec
            .last()
            .cloned()
            .expect("should be valid.");
        let to_divide_prod_info = self.prefix_product_vec[to_divid_idx];
        if to_divide_prod_info.1 < last_prefix_prod_info.1 {
            0
        } else {
            last_prefix_prod_info.0 / to_divide_prod_info.0
        }
    }
}
