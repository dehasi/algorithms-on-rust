pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut sign = 1;

    for x in nums {
        if x == 0 {
            return 0;
        } else if x < 0 {
            sign = -sign;
        }
    }
    return sign;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = array_sign(vec![-1, -2, -3, -4, 3, 2, 1]);
        assert_eq!(result, 1);
    }
}
