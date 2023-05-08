struct Solution {}

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut sum = 0;

        for i in 0..n {
            sum = sum + mat[i][i] + mat[i][n - i - 1];
        }

        if n % 2 == 1 {
            sum -= mat[n / 2][n / 2];
        }
        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::diagonal_sum(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);

        assert_eq!(result, 25);
    }
}
