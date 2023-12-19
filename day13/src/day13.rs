use rayon::iter::{ParallelBridge, ParallelIterator};

struct Matrix {
    data: Vec<char>,
    width: usize,
}

impl Matrix {
    fn get_point(&self, x: usize, y: usize) -> &char {
        assert!(x < self.width, "tried to retrieve x point out of bounds");
        assert!(y < self.height(), "tried to retrieve y point out of bounds");
        &self.data[x + (y * self.width)]
    }

    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn width(&self) -> usize {
        self.width
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let patterns = input.split("\n\n");
    patterns
        .par_bridge()
        .map(parse_pattern)
        .map(|m| score_reflection(&m))
        .sum()
}

fn parse_pattern(p: &str) -> Matrix {
    let mut width = 0;
    let mut data = vec![];
    p.lines().for_each(|l| {
        width = l.len();
        l.chars().for_each(|c| data.push(c));
    });
    Matrix { data, width }
}

fn score_reflection(m: &Matrix) -> usize {
    // try vertical
    'vertical: for x in 1..m.width() {
        let mut r = x;
        let mut l = x - 1;
        loop {
            for y in 0..m.height() {
                if m.get_point(l, y) != m.get_point(r, y) {
                    continue 'vertical;
                }
            }
            if l == 0 || r == m.width() - 1 {
                return x;
            }
            r += 1;
            l -= 1;
        }
    }

    // try horizontal
    'horizontal: for y in 1..m.height() {
        let mut u = y;
        let mut d = y - 1;
        loop {
            for x in 0..m.width() {
                if m.get_point(x, u) != m.get_point(x, d) {
                    continue 'horizontal;
                }
            }
            if d == 0 || u == m.height() - 1 {
                return y * 100;
            }
            u += 1;
            d -= 1;
        }
    }

    panic!("every pattern should have a reflection")
}

fn score_reflection_smudged(m: &Matrix) -> usize {
    // try vertical
    'vertical: for x in 1..m.width() {
        let mut r = x;
        let mut l = x - 1;
        let mut smudged = false;
        loop {
            for y in 0..m.height() {
                if m.get_point(l, y) != m.get_point(r, y) {
                    if !smudged {
                        smudged = true;
                    } else {
                        continue 'vertical;
                    }
                }
            }
            if l == 0 || r == m.width() - 1 {
                return x;
            }
            r += 1;
            l -= 1;
        }
    }

    // try horizontal
    'horizontal: for y in 1..m.height() {
        let mut u = y;
        let mut d = y - 1;
        let mut smudged = false;
        loop {
            for x in 0..m.width() {
                if m.get_point(x, u) != m.get_point(x, d) {
                    if !smudged {
                        smudged = true;
                    } else {
                        continue 'horizontal;
                    }
                }
            }
            if d == 0 || u == m.height() - 1 {
                return y * 100;
            }
            u += 1;
            d -= 1;
        }
    }

    panic!("every pattern should have a reflection")
}

pub fn solve_part_two(_input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EXAMPLE: &str = r#""#;

    const TEST_EXAMPLE_HORIZONTAL: &str = r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    const TEST_EXAMPLE_VERTICAL: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;

    #[test]
    fn matrix_test() {
        let m = parse_pattern(TEST_EXAMPLE_VERTICAL);
        assert_eq!(m.width(), 9);
        assert_eq!(m.height(), 7);
        assert_eq!(m.get_point(0, 0), &'#');
        assert_eq!(m.get_point(8, 6), &'.');
        assert_eq!(m.get_point(2, 0), &'#');
        assert_eq!(m.get_point(2, 4), &'#');
    }

    #[test]
    fn test_vertical_reflection() {
        let m = parse_pattern(TEST_EXAMPLE_VERTICAL);
        assert_eq!(score_reflection(&m), 5);
    }

    #[test]
    fn test_horizontal_reflection() {
        let m = parse_pattern(TEST_EXAMPLE_HORIZONTAL);
        assert_eq!(score_reflection(&m), 400);
    }

    #[test]
    fn test_horizontal_reflection_smudged() {
        let m = parse_pattern(TEST_EXAMPLE_HORIZONTAL);
        assert_eq!(score_reflection_smudged(&m), 300);
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(TEST_EXAMPLE), 21);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_EXAMPLE), 525152);
    }
}
