//! ## Leetcode 2073. Time Needed to Buy Tickets
//! https://leetcode.com/problems/time-needed-to-buy-tickets
//! - `Easy`; `Independently Solved`; `2024-04-08`;
//!
//! Firstly, you can try to put the target customer at the end of the line. Then, the time needed would be related to the sum of the floor-to-target-customer number of tickets.

pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    let k_usize = k as usize;
    let max_need = tickets[k_usize] - 1;
    // let mut counts: Vec<usize> = vec![0; max_need as usize + 1];
    let mut ans: i32 = 0;
    for (i, ticket_need) in tickets.iter().enumerate() {
        let mut curr = *ticket_need;
        if i <= k_usize {
            ans += 1;
            curr -= 1;
        }

        curr = curr.min(max_need);

        ans += curr;
    }

    ans
}
