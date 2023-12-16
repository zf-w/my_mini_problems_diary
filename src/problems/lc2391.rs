//! ## Leetcode 2391. Minimum Amount of Time to Collect Garbage
//! https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage
//! - `Medium`; `Independently Solved`; `2023-11-19`; 
//! 
//! This problem is a bit strange to me. It seems a simulation would be able to solve it. The key to minimizing the time is not letting trucks go to unnecessary stops. 

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