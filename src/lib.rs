mod task1_two_sum;
mod task54_spiral_matrix;
mod task59_spiral_matrix_ii;
mod task649_dota2_senate;
mod task1456_maximum_number_of_vowels_in_a_substring_of_given_length;
mod task1491_average_salary_excluding_the_minimum_and_maximum_salary;
mod task1498_number_of_subsequences_that_satisfy_the_given_sum_condition;
mod task1572_matrix_diagonal_sum;
mod task1822_sign_of_the_product_of_an_array;
mod task2140_solving_questions_with_brainpower;
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
