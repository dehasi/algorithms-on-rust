struct Solution;

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let module = 1_000_000_000 + 7;
        let mut x = nums.to_vec();

        let mut pow2 = vec![1; x.len()];
        for i in 1..x.len() {
            pow2[i] = 2 * pow2[i - 1] % module;
        }
        println!("{:?}", pow2);
        x.sort();
        let mut result = 0;
        let mut l = 0;
        let mut r = (x.len() - 1) as i32;
        while l <= r && r > r - 1 {
            if x[l as usize] + x[r as usize] > target {
                r -= 1;
            } else {
                result = (result + pow2[(r - l) as usize]) % module;
                l += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::num_subseq(vec![3, 5, 6, 7], 9);

        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = Solution::num_subseq(vec![3, 3, 6, 8], 10);

        assert_eq!(result, 6);
    }

    #[test]
    fn it_works3() {
        let result = Solution::num_subseq(vec![2, 3, 3, 4, 6, 7], 12);

        assert_eq!(result, 61);
    }

    #[test]
    fn it_works4() {
        let result = Solution::num_subseq(vec![1], 1);

        assert_eq!(result, 1);
    }
}