struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut dp = vec![0 as i64; n];
        dp[n - 1] = questions[n - 1][0] as i64;

        for i in (0..(n - 1)).rev() {
            let prev_index = questions[i][1] as usize + i + 1 as usize;
            let take = questions[i][0] as i64 + if prev_index < n { dp[prev_index] } else { 0 };
            let skip = dp[i + 1];

            dp[i] = std::cmp::max(take, skip);
        }

        return dp[0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]);

        assert_eq!(res, 5);
    }
}
