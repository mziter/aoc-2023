use itertools::Itertools;
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn solve_part_one(input: &str) -> i64 {
    input.par_lines().map(predict_next).sum()
}

pub fn solve_part_two(input: &str) -> i64 {
    input.par_lines().map(predict_previous).sum()
}

fn predict_previous(history: &str) -> i64 {
    let mut matrix: Vec<Vec<i64>> = Vec::with_capacity(history.len());

    // parse first row
    let first_row = history
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .rev()
        .collect_vec();
    matrix.push(first_row);

    // build down
    let mut all_zeros = false;
    let mut depth = 0;
    while !all_zeros {
        depth += 1;
        let mut next_row: Vec<i64> = Vec::with_capacity(matrix[depth - 1].len() - 1);
        matrix[depth - 1]
            .iter()
            .tuple_windows()
            .for_each(|(a, b)| next_row.push(a - b));
        if next_row.iter().all(|n| n == &0) {
            all_zeros = true;
        }
        matrix.push(next_row);
    }

    // get answer
    // Should I work up or recurse down?
    matrix[depth].push(0);
    // let mut len = matrix[depth].len() + 1;
    while depth > 0 {
        depth -= 1;
        let i = matrix[depth].len() - 1;
        let next = &matrix[depth][i] - &matrix[depth + 1][i];
        matrix[depth].push(next);
    }

    *matrix[0].last().unwrap()
}

fn predict_next(history: &str) -> i64 {
    let mut matrix: Vec<Vec<i64>> = Vec::with_capacity(history.len());

    // parse first row
    let first_row = history
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect_vec();
    matrix.push(first_row);

    // build down
    let mut all_zeros = false;
    let mut depth = 0;
    while !all_zeros {
        depth += 1;
        let mut next_row: Vec<i64> = Vec::with_capacity(matrix[depth - 1].len() - 1);
        matrix[depth - 1]
            .iter()
            .tuple_windows()
            .for_each(|(a, b)| next_row.push(b - a));
        if next_row.iter().all(|n| n == &0) {
            all_zeros = true;
        }
        matrix.push(next_row);
    }

    // get answer
    // Should I work up or recurse down?
    matrix[depth].push(0);
    // let mut len = matrix[depth].len() + 1;
    while depth > 0 {
        depth -= 1;
        let i = matrix[depth].len() - 1;
        let next = &matrix[depth][i] + &matrix[depth + 1][i];
        matrix[depth].push(next);
    }

    *matrix[0].last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EXAMPLE: &str = r#"0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45"#;

    const TEST_PART_TWO_EXAMPLE: &str = r#"10 13 16 21 30 45"#;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(TEST_EXAMPLE), 114);
    }

    #[test]
    fn test_solve_part_two_example() {
        assert_eq!(solve_part_two(TEST_PART_TWO_EXAMPLE), 5);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_EXAMPLE), 2);
    }
}
