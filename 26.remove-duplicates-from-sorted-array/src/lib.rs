pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut idl = 1;
        for index in 1..nums.len() {
            if nums[index - 1] != nums[index] {
                nums[idl] = nums[index];
                idl += 1;
            }
        }

        idl as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![1, 1, 2];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 2);
    }
}
