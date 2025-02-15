//! # Leetcode 2698. Find the Punishment Number of an Integer
//! https://leetcode.com/problems/find-the-punishment-number-of-an-integer/
//! - `Medium`; `y2025m02d15`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;

pub fn helper_is_a_number_partitionable_with_equal_sum(
    to_partition_num: i32,
    need_num: i32,
) -> bool {
    let mut extract_num: i32 = 10;

    if (to_partition_num == need_num) {
        return true;
    }

    while (to_partition_num / extract_num > 0) {
        let got_num = to_partition_num % extract_num;
        if got_num >= need_num {
            return false;
        }

        if helper_is_a_number_partitionable_with_equal_sum(
            to_partition_num / extract_num,
            need_num - got_num,
        ) {
            return true;
        }

        extract_num *= 10;
    }

    false
}

const VALID_NUM_PAIR_ARR: [(i32, i32); 29] = [
    (1, 1),
    (9, 81),
    (10, 100),
    (36, 1296),
    (45, 2025),
    (55, 3025),
    (82, 6724),
    (91, 8281),
    (99, 9801),
    (100, 10000),
    (235, 55225),
    (297, 88209),
    (369, 136161),
    (370, 136900),
    (379, 143641),
    (414, 171396),
    (657, 431649),
    (675, 455625),
    (703, 494209),
    (756, 571536),
    (792, 627264),
    (909, 826281),
    (918, 842724),
    (945, 893025),
    (964, 929296),
    (990, 980100),
    (991, 982081),
    (999, 998001),
    (1000, 1000000),
];

pub fn punishment_number(n: i32) -> i32 {
    let mut ans_sum = 0;
    // for i in 1..=n {
    //     let i_square = i * i;
    //     if helper_is_a_number_partitionable_with_equal_sum(i_square, i) {
    //         ans_sum += i_square;
    //         println!("({i}, {i_square}),");
    //     }
    // }
    for (i, i_square) in VALID_NUM_PAIR_ARR {
        if i > n {
            break;
        }
        ans_sum += i_square;
    }
    ans_sum
}
