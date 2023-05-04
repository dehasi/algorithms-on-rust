use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();

    for x in nums1 { set1.insert(x); }
    for x in nums2 { set2.insert(x); }

    let mut intersection = HashSet::new();
    for x in &set1 {
        if set2.contains(x) {
            intersection.insert(x.clone());
        }
    }

    for x in intersection {
        set1.remove(&x);
        set2.remove(&x);
    }

    return vec![set1.into_iter().collect(), set2.into_iter().collect()];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = find_difference(vec![1, 2, 3], vec![2, 4, 6]);

        assert_eq!(result, vec![vec![3, 1], vec![6, 4]]);
    }
}
