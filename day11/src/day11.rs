struct TupleCombinationIter<'a, T> {
    i: usize,
    j: usize,
    items: &'a [T],
}

impl<'a, T> TupleCombinationIter<'a, T> {
    fn new(items: &[T]) -> TupleCombinationIter<T> {
        TupleCombinationIter { i: 0, j: 1, items }
    }
}

impl<'a, T> Iterator for TupleCombinationIter<'a, T> {
    type Item = (&'a T, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        // reset if at end of items
        if self.j == self.items.len() {
            self.i += 1;
            if self.i >= self.items.len() {
                return None;
            }
            self.j = self.i + 1;
            if self.j >= self.items.len() {
                return None;
            }
        }

        let next_item = (&self.items[self.i], &self.items[self.j]);
        self.j += 1;
        Some(next_item)
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn dist(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let mut cols = vec![1; 140];
    let mut rows = vec![1; 140];
    let mut galaxies = Vec::with_capacity(500);

    input.lines().enumerate().for_each(|(r, row)| {
        row.chars().enumerate().for_each(|(c, ch)| {
            if ch == '#' {
                cols[c] = 0;
                rows[r] = 0;
                galaxies.push(Point { x: c, y: r });
            }
        })
    });

    TupleCombinationIter::new(&galaxies)
        .map(|(a, b)| {
            let additional_y: usize = if a.y > b.y {
                rows[b.y..=a.y].iter().sum()
            } else {
                rows[a.y..=b.y].iter().sum()
            };

            let additional_x: usize = if a.x > b.x {
                cols[b.x..=a.x].iter().sum()
            } else {
                cols[a.x..=b.x].iter().sum()
            };
            a.dist(b) + additional_y + additional_x
        })
        .sum()
}

pub fn solve_part_two(input: &str, expansion_val: usize) -> usize {
    let mut cols = vec![expansion_val - 1; 140];
    let mut rows = vec![expansion_val - 1; 140];
    let mut galaxies = Vec::with_capacity(500);

    input.lines().enumerate().for_each(|(r, row)| {
        row.chars().enumerate().for_each(|(c, ch)| {
            if ch == '#' {
                cols[c] = 0;
                rows[r] = 0;
                galaxies.push(Point { x: c, y: r });
            }
        })
    });

    TupleCombinationIter::new(&galaxies)
        .map(|(a, b)| {
            let additional_y: usize = if a.y > b.y {
                rows[b.y..=a.y].iter().sum()
            } else {
                rows[a.y..=b.y].iter().sum()
            };

            let additional_x: usize = if a.x > b.x {
                cols[b.x..=a.x].iter().sum()
            } else {
                cols[a.x..=b.x].iter().sum()
            };
            a.dist(b) + additional_y + additional_x
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EXAMPLE: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(TEST_EXAMPLE), 374);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_EXAMPLE, 10), 1030);
    }

    #[test]
    fn test_solve_part_two_second() {
        assert_eq!(solve_part_two(TEST_EXAMPLE, 100), 8410);
    }
}
