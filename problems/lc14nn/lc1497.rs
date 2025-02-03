//! ## Leetcode 1497. Check If Array Pairs Are Divisible by k
//! https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/
//! - `Medium`; `Independently Solved`; `20241001`;

pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    let k_usize: usize = k as usize;
    let mut counts_vec: Vec<usize> = vec![0; k_usize];
    // println!("{}", -1 % 5);
    for arr_elem in arr {
        let elem_i = (arr_elem.rem_euclid(k)) as usize;
        *unsafe { counts_vec.get_unchecked_mut(elem_i) } += 1;
    }
    let mut begin_i = 1;
    let mut end_i = k_usize - 1;
    while begin_i < end_i {
        if counts_vec[begin_i] != counts_vec[end_i] {
            return false;
        }
        begin_i += 1;
        end_i -= 1;
    }
    if begin_i == end_i {
        counts_vec[begin_i] == 0
    } else {
        true
    }
}
