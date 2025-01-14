// use std::collections::HashMap;
struct Solution;

impl Solution {
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut map = HashMap::new();

    //     for (i, &num) in nums.iter().enumerate() {
    //         let complement = target - num;
    //         if let Some(&index) = map.get(&complement) {
    //             return vec![index as i32, i as i32];
    //         }
    //         map.insert(num, i);
    //     }
    //     vec![]
    // }
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for x in 0..nums.len() - 1 {
            for y in (x + 1)..nums.len() {
                if nums[x] + nums[y] == target {
                    return vec![x as i32, y as i32];
                }
            }
        }
        return vec![];
    }
}

fn main() {
    let nums = vec![1, 5, 3, 2, 4, 8, 4, 9, 8];
    let target = 13;

    let result = Solution::two_sum(nums, target);
    println!("Output: {:?}", result)
}
