#![allow(dead_code)]

// fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     for i in 0..nums.len() {
//         for j in i + 1..nums.len() {
//             if nums[i] + nums[j] == target {
//                 return vec![i as i32, j as i32];
//             }
//         }
//     }
//     unreachable!();
// }

use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut idl = HashMap::new();
    for (j, &x) in nums.iter().enumerate() {
        if let Some(&i) = idl.get(&(target - x)) {
            return vec![i as i32, j as i32];
        }
        idl.insert(x, j);
    }
    unreachable!();
}

fn main() {
    let arr = two_sum(vec![1, 2, 3, 4], 6);
    println!("{:?}", arr);
}
