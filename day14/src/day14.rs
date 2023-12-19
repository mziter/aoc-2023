use std::vec;

pub fn solve_part_one(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = vec![];

    // count stones until we see a square stone '#'
    input.lines().enumerate().for_each(|(i, line)| {
        grid.push(vec![]);
        for ch in line.chars() {
            grid[i].push(ch);
        }
    });

    let mut sum = 0;
    for c in 0..grid[0].len() {
        let mut row_total = 0;
        let mut load = grid.len();
        for r in 0..grid.len() {
            let spot = grid[r][c];
            if spot == 'O' {
                row_total += load;
                load -= 1;
            }
            if spot == '#' {
                load = grid.len() - r - 1;
            }
        }
        sum += row_total;
    }

    sum
}

pub fn total_load(stones: usize, height: usize) -> usize {
    (0..=height).rev().take(stones).sum()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Point {
    x: usize,
    y: usize,
}

struct Matrix {
    data: Vec<Vec<char>>,
    facing: Direction,
}

impl Matrix {
    fn rotate_counter_clockwise(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    // move all the stones that can move
    fn tilt(&mut self) {}
}

// rethink everything!
//
// array of spaces
//
pub fn solve_part_two(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EXAMPLE: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(TEST_EXAMPLE), 136);
    }
}
