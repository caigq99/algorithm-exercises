use std::collections::HashMap;
pub struct Solution;

impl Solution {
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     for i in 0..nums.len() {
    //         for j in i + 1..nums.len() {
    //             if nums[i] + nums[j] == target {
    //                 return vec![i as i32, j as i32];
    //             }
    //         }
    //     }
    //     unreachable!();
    // }
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut idl = HashMap::new();
        for (j, &x) in nums.iter().enumerate() {
            if let Some(&i) = idl.get(&(target - x)) {
                return vec![i as i32, j as i32];
            }
            idl.insert(x, j);
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn two_sum() {
        let sum = vec![1, 2, 3, 4];
        let target = 6;
        assert_eq!(Solution::two_sum(sum, target), [1, 3]);
    }
}
