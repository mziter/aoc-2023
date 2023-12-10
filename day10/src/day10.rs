use itertools::Itertools;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

#[derive(Debug, Copy, Clone)]
enum PipeType {
    Missing,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn neighbor_iter<'a>(&self) -> NeighborIterator {
        NeighborIterator {
            cycle_n: 0,
            origin: &self,
        }
    }

}

struct Matrix<'a> {
    points: Vec<&'a str>,
}

struct NeighborIterator<'a> {
    cycle_n: usize,
    origin: &'a Point,
}

impl Iterator for NeighborIterator<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.cycle_n >= 4 {
            return None;
        }
        self.cycle_n += 1;
        match self.cycle_n {
            1 => {
                // above
                let y = self.origin.y.checked_sub(1)?;
                Some(Point {
                    x: self.origin.x,
                    y,
                })
            }
            2 => {
                // right
                Some(Point {
                    x: self.origin.x + 1,
                    y: self.origin.y,
                })
            }
            3 => {
                // down
                Some(Point {
                    x: self.origin.x,
                    y: self.origin.y + 1,
                })
            }
            4 => {
                // left
                let x = self.origin.x.checked_sub(1)?;
                Some(Point {
                    x,
                    y: self.origin.y,
                })
            }
            _ => panic!("iterator should never cycle above 4 neighbors"),
        }
    }

    fn connected(&self, tiles: &Matrix) -> NeighborIterator {
        self.filter(|p| p.)
    }
}

impl<'a> Matrix<'a> {
    fn from_str(matrix_data: &'a str) -> Matrix<'a> {
        let len = matrix_data.find("\n").unwrap();
        let mut matrix = Matrix {
            points: Vec::with_capacity(len),
        };

        matrix_data.lines().for_each(|l| matrix.points.push(l));

        matrix
    }

    fn find_start(&self) -> Point {
        for (y, r) in self.points.iter().enumerate() {
            for (x, c) in r.chars().enumerate() {
                if c == 'S' {
                    return Point { x, y };
                }
            }
        }
        panic!("should always find a start value in the data")
    }

    fn connected(&self, a: &Point, b: &Point) -> bool {
        // are they adjacent?
        // does one of the directions match up?
    }
}

pub fn solve_part_one(input: &str) -> i64 {
    let m = Matrix::from_str(input);
    let p = m.find_start();
    p.neighbor_iter().for_each(|p| println!("{:?}", p));

    todo!()
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
