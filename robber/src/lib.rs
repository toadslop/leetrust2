use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        if nums.len() == 2 {
            return max(nums[0], nums[1]);
        }

        let mut chart = [nums[0], max(nums[0], nums[1]), 0];

        for i in 2..nums.len() {
            chart[2] = max(chart[1], nums[i] + chart[0]);
            chart[0] = chart[1];
            chart[1] = chart[2];
        }

        chart[2]
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3, 1];
        let expected = 4;
        let actual = Solution::rob(nums);

        assert_eq!(expected, actual);
    }

    #[test]
    fn example_2() {
        let nums = vec![2, 7, 9, 3, 1];
        let expected = 12;
        let actual = Solution::rob(nums);

        assert_eq!(expected, actual);
    }

    #[test]
    fn example_3() {
        let nums = vec![0];
        let expected = 0;
        let actual = Solution::rob(nums);

        assert_eq!(expected, actual);
    }

    #[test]
    fn example_4() {
        let nums = vec![1, 6];
        let expected = 6;
        let actual = Solution::rob(nums);

        assert_eq!(expected, actual);
    }

    #[test]
    fn example_5() {
        let nums = vec![6, 1];
        let expected = 6;
        let actual = Solution::rob(nums);

        assert_eq!(expected, actual);
    }

    #[test]
    fn example_6() {
        let nums = vec![2, 1, 1, 2];
        let expected = 4;
        let actual = Solution::rob(nums);

        assert_eq!(expected, actual);
    }
}
