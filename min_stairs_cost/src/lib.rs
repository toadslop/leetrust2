use std::cmp::min;

pub struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut results = [0, cost[0], cost[1]];

        for i in 2..cost.len() {
            results[0] = results[1];
            results[1] = results[2];
            results[2] = min(results[1], results[0]) + cost[i];
        }

        min(results[1], results[2])
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn example_1() {
        let cost = vec![10, 15, 20];
        let expected = 15;
        let actual = Solution::min_cost_climbing_stairs(cost);
        assert_eq!(expected, actual)
    }

    #[test]
    fn example_2() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let expected = 6;
        let actual = Solution::min_cost_climbing_stairs(cost);
        assert_eq!(expected, actual)
    }

    #[test]
    fn example_3() {
        let cost = vec![1, 5];
        let expected = 1;
        let actual = Solution::min_cost_climbing_stairs(cost);
        assert_eq!(expected, actual)
    }

    #[test]
    fn example_4() {
        let cost = vec![5, 1];
        let expected = 1;
        let actual = Solution::min_cost_climbing_stairs(cost);
        assert_eq!(expected, actual)
    }

    #[test]
    fn example_5() {
        let cost = vec![1, 2, 3, 4, 5];
        let expected = 6;
        let actual = Solution::min_cost_climbing_stairs(cost);
        assert_eq!(expected, actual)
    }
}
