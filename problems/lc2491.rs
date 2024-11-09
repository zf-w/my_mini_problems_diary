//! ## Leetcode 2491. Divide Players Into Teams of Equal Skill
//! https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/
//! - `Medium`; `Independently Solved`; `y2023m10d04`;

pub fn divide_players(skills_vec: Vec<i32>) -> i64 {
    let len = skills_vec.len();
    let pairs_count = len / 2;

    let mut skills_count_map_arr: [usize; 1000] = [0; 1000];
    let mut sum = 0;
    for skill in skills_vec {
        sum += skill;
        let skill_i = (skill - 1) as usize;
        *skills_count_map_arr
            .get_mut(skill_i)
            .expect("skill[i] <= 1000") += 1;
    }

    let pair_skill = sum / pairs_count as i32;
    let pair_skill_usize = pair_skill as usize;
    let mut ans_mul_sum: i64 = 0;

    for i in 0..1000usize {
        let skill = (i + 1);
        let count_0 = skills_count_map_arr[i];
        if count_0 == 0 {
            continue;
        }
        if skill >= pair_skill_usize && count_0 > 0 {
            return -1;
        }
        let target_skill = pair_skill_usize - skill;
        if target_skill > 1000 {
            return -1;
        }
        let count_1 = skills_count_map_arr[target_skill - 1];
        if count_0 != count_1 {
            return -1;
        }
        skills_count_map_arr[i] = 0;
        skills_count_map_arr[target_skill - 1] = 0;
        // println!("{} {} {}", skill, target_skill, count_0);
        if (skill == target_skill) {
            ans_mul_sum += ((skill * target_skill) * count_0) as i64 / 2;
        } else {
            ans_mul_sum += ((skill * target_skill) * count_0) as i64;
        }
    }
    ans_mul_sum
}
