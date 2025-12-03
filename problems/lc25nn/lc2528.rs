//! # Leetcode 2528. Maximize the Minimum Powered City
//! https://leetcode.com/problems/maximize-the-minimum-powered-city/
//! - `Hard`; `y2025m11d07`; `Learned from Editorial`; `11ms`; `4.8mb`; `4 attempts`;
//! Topics: binary_search,difference_array.

pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
    let half_range_offset = r as usize;
    let full_range_offset = half_range_offset * 2 + 1;
    let new_station_num = k as usize;
    let city_num = stations.len();

    let mut diff_arr_box: Box<[i64]> = vec![0; city_num].into_boxed_slice();

    fn i32_to_i64_fn(num: i32) -> i64 {
        num as i64
    }

    let mut begin_min_power = stations.iter().cloned().min().expect("len > 0") as i64;
    let mut end_min_power = stations.iter().cloned().map(i32_to_i64_fn).sum::<i64>() + k as i64 + 1;

    // println!("{:?}", stations);

    for (city_i, city_station_num) in stations.iter().cloned().map(i32_to_i64_fn).enumerate() {
        let begin_city_i = if city_i <= half_range_offset {
            0
        } else {
            city_i - half_range_offset
        };

        diff_arr_box[begin_city_i] += city_station_num;

        if city_i + half_range_offset + 1 < city_num {
            diff_arr_box[city_i + half_range_offset + 1] -= city_station_num;
        };
    }

    let diff_arr_box = diff_arr_box;

    let check_valid_fn =
        |query_min_power: i64, query_helper_diff_slice_mut_ref: &mut [i64]| -> bool {
            let mut curr_power_num: i64 = 0;
            let mut remain_station_buget = new_station_num as i64;

            query_helper_diff_slice_mut_ref.clone_from_slice(&diff_arr_box);

            for city_i in 0..city_num {
                let diff_entry = query_helper_diff_slice_mut_ref[city_i];
                curr_power_num += diff_entry;

                if curr_power_num >= query_min_power {
                    continue;
                }

                let need_station_num = query_min_power - curr_power_num;

                if need_station_num > remain_station_buget {
                    return false;
                }

                remain_station_buget -= need_station_num;
                curr_power_num = query_min_power;

                if city_i + full_range_offset >= city_num {
                    continue;
                }

                query_helper_diff_slice_mut_ref[city_i + full_range_offset] -= need_station_num;
            }

            true
        };

    let mut query_help_arr_box_mut_ref = vec![0; city_num].into_boxed_slice();

    while begin_min_power < end_min_power {
        let mid_query_min_power = (begin_min_power + end_min_power) / 2;
        let valid_flag = check_valid_fn(mid_query_min_power, &mut query_help_arr_box_mut_ref);

        // println!("{} {}", mid_query_min_power, valid_flag);

        if valid_flag == true {
            begin_min_power = mid_query_min_power + 1;
        } else {
            end_min_power = mid_query_min_power;
        }
    }

    begin_min_power - 1
}