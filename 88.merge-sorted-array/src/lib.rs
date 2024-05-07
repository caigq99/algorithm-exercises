pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // 如果n为0 直接return  保持nums1不变
        if n == 0 {
            return;
        }
        // 如果m为零 代表nums1的实际长度为0 直接将nums2的元素逐个替换即可
        if m == 0 {
            for (index, val) in nums2.iter().enumerate() {
                nums1[index] = *val
            }
            return;
        }
        let mut p1 = m - 1;
        let mut p2 = n - 1;
        let mut p3 = (m + n - 1) as usize;
        while p1 >= 0 && p2 >= 0 {
            if nums1[p1 as usize] < nums2[p2 as usize] {
                nums1[p3] = nums2[p2 as usize];
                p2 -= 1;
            } else if nums1[p1 as usize] >= nums2[p2 as usize] {
                nums1[p3] = nums1[p1 as usize];
                p1 -= 1;
            }
            p3 -= 1;
        }
        while p2 >= 0 {
            nums1[p3] = nums2[p2 as usize];
            p2 -= 1;
            p3 -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let m = 3;
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
    #[test]
    fn it_works2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        let m = 1;
        let n = 0;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }
    #[test]
    fn it_works3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        let m = 0;
        let n = 1;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }
}
