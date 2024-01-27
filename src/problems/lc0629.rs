pub fn k_inverse_pairs(ni: i32, ki: i32) -> i32 {
    let n = ni as usize;
    let k = ki as usize;
    let mut dp: Vec<i32> = vec![0; n as usize * (k + 1) as usize];
    let index = |i, j| -> usize { i + j * (k + 1) };
    dp[0] = 1;
    let md = 1000_000_007;
    let add = |a: &mut i32, b: i32| {
        *a = (*a + b) % md;
    };
    for i in 1..=n {
        dp[index(i, 0)] = 1;
    }

    for i in 1..=n {
        for j in 0..=k {
            let mut curr = (dp[index(i - 1, j)] + dp[index(i, j - 1)]) % md;
            if j >= i {
                add(&mut curr, -dp[index(i - 1, j - i)] + md);
            }
            dp[index(i, j)] = curr;
        }
    }
    dp[index(n, k)]
}
