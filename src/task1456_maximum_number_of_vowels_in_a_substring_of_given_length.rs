use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn max_vowels(ss: String, k: i32) -> i32 {
        fn is_vowel(c: char) -> bool {
            return "aeiou".contains(c);
        }

        let mut cur = 0;
        let mut max = 0;

        let s: Vec<char> = ss.chars().collect();
        for i in 0..k as usize {
            if is_vowel(s[i]) {
                cur += 1;
            }
        }
        max = cmp::max(max, cur);
        for i in (k as usize)..s.len() {
            if is_vowel(s[i - k as usize]) {
                cur -= 1;
            }
            if is_vowel(s[i]) {
                cur += 1;
            }
            max = cmp::max(max, cur);
        }
        return max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::max_vowels("abciiidef".to_string(), 3);

        assert_eq!(result, 3);
    }
}
