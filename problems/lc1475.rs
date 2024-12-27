//! # Leetcode 1475. Final Prices With a Special Discount in a Shop
//! https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/
//! - `Easy`; `y2024m12d26`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;

pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut ans_vec: Vec<i32> = Vec::with_capacity(prices.len());
    let mut price_iter = prices.iter().cloned();
    while let Some(price) = price_iter.next() {
        let next_price_iter = price_iter.clone();
        let mut ans_price = price.clone();
        for next_price in next_price_iter {
            if next_price <= price {
                ans_price -= next_price;
                break;
            }
        }
        ans_vec.push(ans_price);
    }
    ans_vec
}
