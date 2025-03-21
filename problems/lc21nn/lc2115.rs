//! # Leetcode 2115. Find All Possible Recipes from Given Supplies
//! https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/
//! - `Medium`; `y2025m03d21`; `Independently Solved`; `41ms`; `3.2mb`; `1 attempt`;
//! Topics: graph/topology_sort.
//!
//! This problem took me some time to solve.

pub fn find_all_recipes(
    mut recipes: Vec<String>,
    ingredients: Vec<Vec<String>>,
    supplies: Vec<String>,
) -> Vec<String> {
    use std::collections::{HashMap, VecDeque};
    let recipe_num = recipes.len();
    let mut ingredient_map: HashMap<&String, usize> = HashMap::with_capacity(recipe_num);

    use std::collections::hash_map::Entry;

    let mut next_free_idx: usize = 0;

    for ingredient_arr_ref in ingredients.iter().map(|v| -> &[String] { v }) {
        for ingredient_ref in ingredient_arr_ref {
            if let Entry::Vacant(vacant_entry) = ingredient_map.entry(ingredient_ref) {
                vacant_entry.insert(next_free_idx);
                next_free_idx += 1;
            }
        }
    }

    for recipe_ref in recipes.iter() {
        if let Entry::Vacant(vacant_entry) = ingredient_map.entry(recipe_ref) {
            vacant_entry.insert(next_free_idx);
            next_free_idx += 1;
        }
    }

    let node_num = next_free_idx;

    let mut adj_vec: Vec<Vec<usize>> = vec![Vec::new(); node_num];

    let mut info_vec: Vec<(bool, usize)> = vec![(false, 0); node_num];

    for (recipe_idx, ingredient_arr_ref) in
        ingredients.iter().map(|v| -> &[String] { v }).enumerate()
    {
        let recipe_node_idx = ingredient_map
            .get(&recipes[recipe_idx])
            .expect("Should have")
            .clone();
        for ingredient_ref in ingredient_arr_ref {
            let ingredient_node_idx = ingredient_map
                .get(ingredient_ref)
                .expect("Should have")
                .clone();

            adj_vec[ingredient_node_idx].push(recipe_node_idx);

            info_vec[recipe_node_idx].1 += 1;
        }
    }

    // let mut recipe_checklist_vec: Vec<usize> = vec![0; recipe_num];

    let adj_vec = adj_vec;

    let mut node_idx_queue: VecDeque<usize> = VecDeque::with_capacity(next_free_idx);

    for supply_ref in supplies.iter() {
        let node_idx = if let Some(node_idx) = ingredient_map.get(supply_ref) {
            node_idx.clone()
        } else {
            continue;
        };

        info_vec[node_idx].0 = true;

        node_idx_queue.push_back(node_idx);
    }

    while node_idx_queue.is_empty() == false {
        let node_idx = node_idx_queue.pop_front().expect("checked len");

        let next_idx_arr_ref = adj_vec[node_idx].as_slice();

        for next_node_idx in next_idx_arr_ref.iter().cloned() {
            let next_node_ref = &mut info_vec[next_node_idx];

            if next_node_ref.0 == true {
                continue;
            }

            next_node_ref.1 -= 1;

            if next_node_ref.1 == 0 {
                next_node_ref.0 = true;

                node_idx_queue.push_back(next_node_idx);
            }
        }
    }

    let mut ans_vec: Vec<String> = Vec::with_capacity(recipe_num);

    for recipe_ref in recipes.iter() {
        let recipe_node_idx = ingredient_map.get(recipe_ref).expect("Should have").clone();
        if info_vec[recipe_node_idx].0 == false {
            continue;
        }

        ans_vec.push(recipe_ref.clone());
    }

    ans_vec
}
