use itertools::Itertools;
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn solve_part_one(input: &str) -> i64 {
    todo!();
}

pub fn solve_part_two(input: &str) -> i64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EXAMPLE: &str = r#""#;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(TEST_EXAMPLE), 114);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_EXAMPLE), 2);
    }
}
