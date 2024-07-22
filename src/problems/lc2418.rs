//! Leetcode 2418. Sort the People
//! https://leetcode.com/problems/sort-the-people/
//! - `Easy`; `Independently Solved`; `2024-07-22`;

pub fn sort_people(mut names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let len = heights.len();
    let mut helper_vec: Vec<(i32, usize)> = Vec::with_capacity(len);
    for (i0, height) in heights.iter().cloned().enumerate() {
        helper_vec.push((height, i0));
    }
    helper_vec.sort_unstable();
    let mut who_to_place_map: Vec<usize> = Vec::with_capacity(len);
    let mut place_to_who_map: Vec<usize> = Vec::with_capacity(len);
    for i in 0..len {
        who_to_place_map.push(i);
        place_to_who_map.push(i);
    }
    for (place_i, (_, who_move_in_i)) in helper_vec.iter().cloned().enumerate() {
        let place_i = len - 1 - place_i;
        let move_in_from_i = who_to_place_map[who_move_in_i];
        let who_hold_place_i = place_to_who_map[place_i];

        names.swap(place_i, move_in_from_i);
        who_to_place_map[who_hold_place_i] = move_in_from_i;
        // who_to_place_map[who_move_in_i] = place_i;
        place_to_who_map[move_in_from_i] = who_hold_place_i;
        // place_to_who_map[place_i] = who_move_in_i
    }
    names
}
