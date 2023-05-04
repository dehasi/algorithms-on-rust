mod task1822_sign_of_the_product_of_an_array;
mod task1491_average_salary_excluding_the_minimum_and_maximum_salary;
mod task2215_find_the_difference_of_two_arrays;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
