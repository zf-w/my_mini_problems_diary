//! # Leetcode 3531. Count Covered Buildings
//! https://leetcode.com/problems/count-covered-buildings/
//! - `Medium`; `y2025m12d11`; `Hinted`; `81ms`; `12.13mb`; `1 attempt`;
//! 

pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    fn building_vec_to_building_pair_fn(building_vec: &Vec<i32>) -> (i32, i32) {
        (building_vec[0], building_vec[1])
    }

    let building_num = buildings.len();

    let mut x_to_y_bound_map: HashMap<i32, (i32, i32)> = HashMap::with_capacity(building_num);

    let mut y_to_x_bound_map: HashMap<i32, (i32, i32)> = HashMap::with_capacity(building_num);

    fn update_bound_map(bound_map_mut_ref: &mut HashMap<i32, (i32, i32)>, key: i32, val: i32) {
        bound_map_mut_ref
            .entry(key)
            .and_modify(|(lower_in_bound_mut_ref, upper_in_bound_mut_ref)| {
                *lower_in_bound_mut_ref = (*lower_in_bound_mut_ref).min(val);
                *upper_in_bound_mut_ref = (*upper_in_bound_mut_ref).max(val);
            })
            .or_insert((val, val));
    }

    for (building_x, building_y) in buildings.iter().map(building_vec_to_building_pair_fn) {
        update_bound_map(&mut x_to_y_bound_map, building_x, building_y);
        update_bound_map(&mut y_to_x_bound_map, building_y, building_x);
    }

    let mut ans_count = 0;

    fn check_within_bound(bound_map_ref: &HashMap<i32, (i32, i32)>, key: i32, val: i32) -> bool {
        if let Some((lower_in_bound, upper_in_bound)) = bound_map_ref.get(&key).cloned() {
            val > lower_in_bound && val < upper_in_bound
        } else {
            false
        }
    }

    for (building_x, building_y) in buildings.iter().map(building_vec_to_building_pair_fn) {
        let y_within_bound = check_within_bound(&x_to_y_bound_map, building_x, building_y);
        let x_within_bound = check_within_bound(&y_to_x_bound_map, building_y, building_x);

        if x_within_bound && y_within_bound {
            ans_count += 1;
        }
    }

    ans_count
}