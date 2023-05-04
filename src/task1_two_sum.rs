use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        map.insert(nums[i], i);
    }

    for i in 0..nums.len() {
        let c = target - nums[i];
        if map.contains_key(&c) {
            let ii = *map.get(&c).unwrap();
            if ii != i {
                return vec![i as i32, ii as i32];
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = two_sum(vec![2, 7, 11, 15], 9);

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn it_works2() {
        let result = two_sum(vec![3, 2, 4], 6);

        assert_eq!(result, vec![1, 2]);
    }
}
