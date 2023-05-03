pub fn average(salary: Vec<i32>) -> f64 {
    let mut min = salary[0];
    let mut max = salary[0];
    let mut sum = 0 as f64;
    let len = salary.len();

    for x in salary {
        if min > x { min = x; }
        if max < x { max = x; }
        sum += x as f64;
    }
    sum -= min as f64;
    sum -= max as f64;

    return sum / ((len - 2) as f64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = average(vec![4000, 3000, 1000, 2000]);
        assert_eq!(result, 2500.00000);
    }
}
