pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow_index = 0;
        for index in 0..nums.len() {
            if nums[index] != val {
                nums[slow_index] = nums[index];
                slow_index += 1;
            }
        }
        slow_index as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        assert_eq!(Solution::remove_element(&mut nums, val), 2)
    }
}
