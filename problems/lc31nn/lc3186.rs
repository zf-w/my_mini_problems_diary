//! # Leetcode 3186. Maximum Total Damage With Spell Casting
//! https://leetcode.com/problems/maximum-total-damage-with-spell-casting/
//! - `Medium`; `y2025m10d11`; `Learned from Solution`; `100ms`; `6.4mb`; `1 attempt`;
//! Topics: dynamic_programming.
//! https://leetcode.com/problems/maximum-total-damage-with-spell-casting/solutions/7259292/maximum-total-damage-with-spell-casting

pub fn maximum_total_damage(power_vec: Vec<i32>) -> i64 {
            use std::collections::BTreeMap;
    let mut count_map: BTreeMap<i32, usize> = BTreeMap::new();

    for power in power_vec {
        *count_map.entry(power).or_insert(0) += 1;
    }

    let mut dp_vec: Vec<i64> = vec![0; count_map.len()];

    let unique_power_num = count_map.len();

    let count_vec: Vec<(i32, usize)> = count_map.into_iter().collect();

    
    let mut j = 0;
    let mut curr_max: i64 = 0;
    let mut ans_max = 0;
    for i in 0..unique_power_num {
        let (curr_power, curr_count) = count_vec[i];
        
        while j < i && count_vec[j].0 < curr_power - 2 {
            curr_max = curr_max.max(dp_vec[j]);
            j += 1;
        }

        dp_vec[i] = curr_max + (curr_power as i64) * (curr_count as i64);
        ans_max = ans_max.max(dp_vec[i]);
    }

    ans_max
}