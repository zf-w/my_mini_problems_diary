//! ## Leetcode 935. Knight Dialer
//! - `Medium`
//! 
//! The chess knight has a **unique movement**, it may move two squares vertically and one square horizontally, or two squares horizontally and one square vertically (with both forming shape of an L). The possible movements of chess knight are shown in this diagram:
//! 
//! A chess knight can move as indicated in the chess diagram below:
//! 
//! ![Chess knight](https://assets.leetcode.com/uploads/2020/08/18/chess.jpg)
//! 
//! We have a chess knight and a phone pad as shown below, the knight **can only stand on a numeric cell** (i.e. blue cell).
//! 
//! ![Phone pad](https://assets.leetcode.com/uploads/2020/08/18/phone.jpg)
//! 
//! Given an integer `n`, return how many distinct phone numbers of length `n` we can dial.
//! 
//! You are allowed to place the knight **on any numeric cell** initially and then you should perform `n-1` jumps to dial a number of lenght `n`. All jumps should be **valid** knight jumps.
//! 
//! As the answer may be very large, **reutrn the answer modulo `10^9 + 7`.
//! 
//! ### Thoughts:
//! - `Independently Solved`; 2023-11-26;
//! 
//! Ah, quick power and matrix multiplication, some dead memories start to attack me. Personally, I tried to use those fixed arrays on Stack in this problem, and it was a very interesting and smooth experience.

pub fn knight_dialer(n: i32) -> i32 {
    let mut v: [i32; 10] = [1,1,1,1,1,1,1,1,1,1];
    let mut a: [i32; 100] = [
        0,0,0,0,1,0,1,0,0,0,
        0,0,0,0,0,0,1,0,1,0,
        0,0,0,0,0,0,0,1,0,1,
        0,0,0,0,1,0,0,0,1,0,
        1,0,0,1,0,0,0,0,0,1,
        0,0,0,0,0,0,0,0,0,0,
        1,1,0,0,0,0,0,1,0,0,
        0,0,1,0,0,0,1,0,0,0,
        0,1,0,1,0,0,0,0,0,0,
        0,0,1,0,1,0,0,0,0,0
    ];

    fn quick_pow(mut a: [i32; 100], mut p: i32) -> [i32; 100] {
      let mut ans: [i32; 100] = [0; 100];
      let len: usize = 10;
      for i in 0..len {
        ans[i * len + i] = 1;
      }
      fn mutiply(a: [i32; 100], b: [i32; 100]) -> [i32; 100] {
        let m: u64 = 1000000007;
        let mut ans: [i32; 100] = [0; 100];
        let len: usize = 10;
        for i in 0..len {
          for j in 0..len {
            for k in 0..len {
              let m1 =  a[i * len + k] as u64;
              let m2 = b[k * len + j] as u64;
              ans[i * len + j] = ((ans[i * len + j] as u64 + ((m1 * m2) % m)) % m) as i32;
            }
          }
        }
        ans
      }
      while p > 0 {
        if p & 1 > 0 {
          ans = mutiply(a, ans);
        }
        a = mutiply(a, a);
        p = p >> 1;
      }
      ans
    }

    fn mutiply(a: [i32; 100], v: [i32; 10]) -> [i32; 10] {
        let m: u64 = 1000000007;
        let mut ans: [i32; 10] = [0,0,0,0,0,0,0,0,0,0];
        let len: usize = 10;
        for k in 0..len {
            for i in 0..len {
                ans[k] =  ((ans[k] as u64 + ((a[k * 10 + i] as u64 * v[i] as u64) % m)) % m) as i32;
            }
        }
        ans
    }
    a = quick_pow(a, n - 1);
    
    v = mutiply(a, v);
    
    let m: u64 = 1000000007;
    let mut ans: u64 = 0;
    for i in v.iter() {
        ans = (ans + *i as u64) % m;
    }
    ans as i32
}