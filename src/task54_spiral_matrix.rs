struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        enum Direction {
            LeftRight,
            TopBottom,
            BottomTop,
            RightLeft,
        }

        let mut result = Vec::new();

        let mut left = 0;
        let mut top = 0;
        let mut right = matrix[0].len() - 1;
        let mut bottom = matrix.len() - 1;

        let mut direction = Direction::LeftRight;
        while left <= right && top <= bottom {
            match direction {
                Direction::LeftRight => {
                    for i in left..=right {
                        result.push(matrix[top][i]);
                    }
                    top += 1;
                    direction = Direction::TopBottom;
                }
                Direction::TopBottom => {
                    for i in top..=bottom {
                        result.push(matrix[i][right]);
                    }
                    if right == 0 {
                        break;
                    }
                    right -= 1;
                    direction = Direction::RightLeft;
                }
                Direction::RightLeft => {
                    for i in (left..=right).rev() {
                        result.push(matrix[bottom][i]);
                    }
                    bottom -= 1; // should I also make (bottom == 0) check?
                    direction = Direction::BottomTop;
                }
                Direction::BottomTop => {
                    for i in (top..=bottom).rev() {
                        result.push(matrix[i][left]);
                    }
                    left += 1;
                    direction = Direction::LeftRight;
                }
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let resul = Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12]]);

        assert_eq!(resul, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }

    #[test]
    fn it_works2() {
        let resul = Solution::spiral_order(vec![
            vec![3],
            vec![2]]);

        assert_eq!(resul, vec![3, 2]);
    }

    #[test]
    fn it_works3() {
        let resul = Solution::spiral_order(vec![
            vec![1, 2, 3], ]);

        assert_eq!(resul, vec![1, 2, 3]);
    }
}
