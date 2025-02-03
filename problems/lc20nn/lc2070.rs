/// ## Leetcode 2070. Most Beautiful Item for Each Query
/// https://leetcode.com/problems/most-beautiful-item-for-each-query/
/// - `Medium`; `Independently Solved`; `y2024m11d14`;
///
/// 5 minutes before the deadline, I was still finding the bug. So, I copied a solution, submitted it, and solved my bug.
///
/// Reference(even though I didn't read it): https://leetcode.com/problems/most-beautiful-item-for-each-query/solutions/6035442/beats-100-00-for-loop-explained-with-example
///
/// The bug was an off-by-one error in the handling of the Binary Search results. The result is actually an index of the upper bound...?
pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut item_vec: Vec<(i32, i32)> = items
        .into_iter()
        .map(|val| -> (i32, i32) {
            let [price_i32, beauty_i32] = val[..] else {
                unreachable!()
            };
            (price_i32, beauty_i32)
        })
        .collect();
    item_vec.sort_unstable();
    let mut item_iter_mut = item_vec.iter_mut();
    let mut beauty_max = item_iter_mut.next().expect("len >= 1").1;

    for (price_i32_mut_ref, beauty_i32_mut_ref) in item_iter_mut {
        beauty_max = beauty_max.max(beauty_i32_mut_ref.clone());
        *beauty_i32_mut_ref = beauty_max;
        // println!("{} {}", price_i32_mut_ref, beauty_i32_mut_ref);
    }

    let mut ans_vec: Vec<i32> = Vec::with_capacity(queries.len());
    let len = item_vec.len();
    for query_price_i32 in queries {
        let mut begin_i = 0;
        let mut end_i = len;

        while begin_i < end_i {
            let mid_i = (begin_i + end_i) / 2;
            let mid_price_i32 = item_vec[mid_i].0;
            // println!("{} {} {}", begin_i, end_i, mid_price_i32);
            if mid_price_i32 <= query_price_i32 {
                begin_i = mid_i + 1;
            } else {
                end_i = mid_i;
            }
        }
        // println!("-- {}", begin_i);
        // if begin_i < len {
        //     println!("ANS: {} {}", item_vec[begin_i].0, query_price_i32);
        // }
        if begin_i > 0 && item_vec[begin_i - 1].0 <= query_price_i32 {
            // Bug here
            ans_vec.push(item_vec[begin_i - 1].1);
        } else {
            ans_vec.push(0);
        }
    }
    ans_vec
}
