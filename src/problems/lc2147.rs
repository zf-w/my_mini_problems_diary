//! ## Leetcode 2147. Number of Ways to Divide a Long Corridor
//! https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor
//! - `Hard`; `Independently Solved`; `2023-11-27`;
//! 
//! The only way you can make decisions about where to put the room dividers is the space between the second seat of the previous room and the first seat of the current room. I used this intuition to solve the problem.
//! 

pub fn number_of_ways(corridor: String) -> i32 {
    let mut prev_seat: usize = 0;
    let mut seat_count: usize = 0;
    let mut ans: i32 = 1;
    let m: usize = 1000000007;
    for (i,c) in corridor.char_indices() {
        match c {
        'S' => {
            seat_count += 1;
            if seat_count > 2 && seat_count & 1 == 1 {
                ans = ((ans as usize * (i - prev_seat)) % m) as i32;
            }
            
            prev_seat = i;
        },
        _ => ()
        }
    }
    
    if seat_count % 2 == 1 || seat_count == 0 {
        0
    } else {
        ans
    }
}