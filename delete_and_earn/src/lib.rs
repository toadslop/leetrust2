use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut max = 0;
        unimplemented!()
    }

    fn delve(nums: Vec<i32>, target: i32) -> i32 {
        let nums = nums
            .iter()
            .filter(|&num| *num == target - 1 || *num == target + 1)
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let nums = vec![3, 4, 2];
        let expected = 6;
        let actual = Solution::delete_and_earn(nums);
        assert_eq!(expected, actual)
    }

    #[test]
    fn t2() {
        let nums = vec![2, 2, 3, 3, 3, 4];
        let expected = 9;
        let actual = Solution::delete_and_earn(nums);
        assert_eq!(expected, actual)
    }
}
