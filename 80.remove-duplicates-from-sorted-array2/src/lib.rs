pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 2 {
            return len as i32;
        }
        let mut slow = 2;
        for fast in 2..len {
            if nums[slow - 2] != nums[fast] {
                nums[slow] = nums[fast];
                slow += 1;
            }
        }
        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 5);
    }
}
