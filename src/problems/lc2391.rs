//! ## Leetcode 2391. Minimum Amount of Time to Collect Garbage
//! `Medium`
//! 
//! You are given a `0-indexed` array of strings `garbage` where `garbage[i]` represents the assortment of garbage at the `ith` house. `garbage[i]` consists only of the characters `'M'`, `'P'`, and `'G'` representing one unit of metal, paper and glass garbage respectively. Picking up **one** unit of any type of garbage takes `1` mintue.
//! 
//! You are also given a **0-indexed** integer array `travel` where `travel[i]` is the number of minutes needed to go from house `i` to the house `i + 1`.
//! 
//! There are three garbage trucks in the city, each responsible for picking up one type of garbage. Each garbage truck starts at house `0` and must visit each house **in order**; however, they do **not** need to visit every house.
//! 
//! Only **one** garbage truck may be used at any given moment. While one truck is driving or picking up garbage, the other two trucks cannot do anything.
//! 
//! Return *the **minimum** number of minuts needed to pick up all the garbage*.
//! 
//! ### Example:
//! 
//! ```
//! use learn_cs::problems::lc2391;
//! 
//! let garbage: Vec<String> = vec![
//!   String::from("G"), 
//!   String::from("P"), 
//!   String::from("GP"), 
//!   String::from("GG")];
//! let travel: Vec<i32> = vec![2, 4, 3];
//! 
//! assert_eq!(21, lc2391::garbage_collection(garbage, travel));
//! 
//! ```
//! 
//! ### Thoughts:
//! - `Independently Solved`; 2023-11-19; 
//! 
//! This problem is a bit strange to me. It seems a simulation would be able to solve it. The key to minimizing the time is not letting trucks go to unnecessary stops. 
//! 

pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    let len: usize = garbage.len();
    let mut dis: (u32, u32, u32) = (0, 0, 0);
    let mut ans: u32 = 0;
    for (i, s) in garbage.iter().enumerate() {
        let mut need: u8 = 0;
        ans += s.len() as u32;
        for c in s.chars() {
            match c {
                'M' => {
                    need |= 1;
                },
                'P' => {
                    need |= 2;
                },
                'G' => {
                    need |= 4;
                },
                _ => {},
            }
            if need == 7 {
                break;
            }
        }
        if (need & 1) > 0 {
            ans += dis.0;
            dis.0 = 0;
        }
        if (need & 2) > 0 {
            ans += dis.1;
            dis.1 = 0;
        }
        if (need & 4) > 0 {
            ans += dis.2;
            dis.2 = 0;
        }
        if i < len - 1 {
            let curr_dis = travel[i] as u32;
            dis.0 += curr_dis;
            dis.1 += curr_dis;
            dis.2 += curr_dis;
        }
    }

    ans as i32
} 