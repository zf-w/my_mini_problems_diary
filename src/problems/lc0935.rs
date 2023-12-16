//! ## Leetcode 935. Knight Dialer
//! https://leetcode.com/problems/knight-dialer
//! - `Medium`; `Independently Solved`; `2023-11-26`;
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