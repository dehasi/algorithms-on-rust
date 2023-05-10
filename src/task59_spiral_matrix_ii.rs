struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let nn = n as usize;
        let mut matrix = vec![vec![0; nn]; nn];


        let mut top = 0;
        let mut bottom = nn - 1;
        let mut left = 0;
        let mut right = nn - 1;

        let size = n * n;
        let mut value = 1;
        while value <= size {
            for i in left..=right {
                matrix[top][i] = value;
                value += 1;
            }
            top += 1;
            if value > size { break; }
            for i in top..=bottom {
                matrix[i][right] = value;
                value += 1;
            }
            right -= 1;
            if value > size { break; }
            for i in (left..=right).rev() {
                matrix[bottom][i] = value;
                value += 1;
            }
            bottom -= 1;
            if value > size { break; }
            for i in (top..=bottom).rev() {
                matrix[i][left] = value;
                value += 1;
            }
            left += 1;
        }
        return matrix;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = Solution::generate_matrix(1);

        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn it_works2() {
        let result = Solution::generate_matrix(2);

        assert_eq!(result, vec![
            vec![1, 2],
            vec![4, 3]]);
    }

    #[test]
    fn it_works3() {
        let result = Solution::generate_matrix(3);

        assert_eq!(result, vec![
            vec![1, 2, 3],
            vec![8, 9, 4],
            vec![7, 6, 5]]);
    }
}
