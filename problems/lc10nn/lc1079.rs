//! # Leetcode 1079. Letter Tile Possibilities
//! https://leetcode.com/problems/letter-tile-possibilities/
//! - `Medium`; `y2025m02d16`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;

fn num_tile_possibilities_helper(tile_count_arr_mut_ref: &mut [usize; 26], ans_mut_ref: &mut i32) {
    *ans_mut_ref += 1;
    for idx in 0..26 {
        if tile_count_arr_mut_ref[idx] == 0 {
            continue;
        }
        tile_count_arr_mut_ref[idx] -= 1;

        num_tile_possibilities_helper(tile_count_arr_mut_ref, ans_mut_ref);

        tile_count_arr_mut_ref[idx] += 1;
    }
}

const CHAR_UPPER_A_U8: u8 = b'A';

pub fn num_tile_possibilities(tiles: String) -> i32 {
    let mut tile_count_arr: [usize; 26] = [0; 26];

    for c_u8 in tiles.as_bytes() {
        tile_count_arr[(*c_u8 - CHAR_UPPER_A_U8) as usize] += 1;
    }

    let mut ans_num = 0;

    num_tile_possibilities_helper(&mut tile_count_arr, &mut ans_num);

    ans_num
}
