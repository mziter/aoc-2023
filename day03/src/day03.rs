use core::iter::Peekable;
use core::str::Chars;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

struct MaxtrixIterator<'a> {
    data: Peekable<Chars<'a>>,
    row: usize,
    col: usize,
    digit_acc: String,
}

fn matrix_iterator(input: &str) -> MaxtrixIterator {
    MaxtrixIterator {
        data: input.chars().peekable(),
        row: 0,
        col: 0,
        digit_acc: "".to_string(),
    }
}

impl Iterator for MaxtrixIterator<'_> {
    type Item = MatrixElement;

    fn next(&mut self) -> Option<MatrixElement> {
        let next = self.data.next();
        next?;

        match next.unwrap() {
            '\n' => {
                self.row += 1;
                self.col = 0;
                self.next()
            }
            c if c.is_ascii_digit() => {
                self.digit_acc.push(c);
                match self.data.peek() {
                    Some(pc) if pc.is_ascii_digit() => {
                        self.col += 1;
                        self.next()
                    }
                    _ => {
                        let digit_range = DigitRange {
                            value: self
                                .digit_acc
                                .parse::<u32>()
                                .expect("should be a string of digits only"),
                            y: self.row,
                            high_x: self.col,
                            low_x: self.col - (self.digit_acc.len() - 1),
                        };
                        self.col += 1;
                        self.digit_acc.clear();
                        Some(MatrixElement::DigitRange(digit_range))
                    }
                }
            }
            c if c != '.' => {
                let symbol = Symbol {
                    val: c,
                    pos: Position {
                        x: self.col,
                        y: self.row,
                    },
                };
                self.col += 1;
                Some(MatrixElement::Symbol(symbol))
            }
            '.' => {
                self.col += 1;
                self.next()
            }
            c => {
                unreachable!(
                    "at character: {}, but all characters should be ., digit, or newline",
                    c
                )
            }
        }
    }
}

enum MatrixElement {
    DigitRange(DigitRange),
    Symbol(Symbol),
}

impl DigitRange {
    pub fn surrounding_positions(&self) -> Vec<Position> {
        let len = (self.high_x + 1) - self.low_x;
        let mut positions = Vec::with_capacity((2 * (len + 2)) + 2);
        let low_y = if self.y == 0 { 0 } else { self.y - 1 };
        let high_y = self.y + 2;
        for y in low_y..high_y {
            let low_x = if self.low_x == 0 { 0 } else { self.low_x - 1 };
            let high_x = self.high_x + 2;
            for x in low_x..high_x {
                // don't include the position of ourselves
                if y == self.y && x >= self.low_x && x <= self.high_x {
                    continue;
                }
                positions.push(Position { x, y })
            }
        }
        positions
    }
}

impl Symbol {
    pub fn surrounding_positions(&self) -> Vec<Position> {
        let mut positions = Vec::with_capacity(8);
        let low_x = if self.pos.x == 0 { 0 } else { self.pos.x - 1 };
        let high_x = self.pos.x + 2;
        for x in low_x..high_x {
            let low_y = if self.pos.y == 0 { 0 } else { self.pos.y - 1 };
            let high_y = self.pos.y + 2;
            for y in low_y..high_y {
                if x == self.pos.x && y == self.pos.y {
                    continue;
                }
                positions.push(Position { x, y })
            }
        }
        positions
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct DigitRange {
    value: u32,
    low_x: usize,
    high_x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Symbol {
    val: char,
    pos: Position,
}

pub fn solve_part_one(input: &str) -> u32 {
    let mut symbol_positions = HashSet::with_capacity(1000);
    let mut digit_ranges = Vec::with_capacity(1500);

    matrix_iterator(input).for_each(|e| match e {
        MatrixElement::Symbol(s) => {
            symbol_positions.insert(s.pos);
        }
        MatrixElement::DigitRange(dr) => {
            digit_ranges.push(dr);
        }
    });

    digit_ranges
        .par_iter()
        .filter(|dr| {
            dr.surrounding_positions()
                .iter()
                .any(|pos| symbol_positions.contains(pos))
        })
        .map(|dr| dr.value)
        .sum()
}

pub fn solve_part_two(input: &str) -> u32 {
    let mut digit_ranges = Vec::with_capacity(1000);
    let mut digit_positions = HashMap::with_capacity(1000);
    let mut gears = Vec::with_capacity(1000);

    matrix_iterator(input).for_each(|e| match e {
        MatrixElement::Symbol(s) => {
            if s.val == '*' {
                gears.push(s)
            }
        }
        MatrixElement::DigitRange(dr) => {
            digit_ranges.push(dr);
            for x in dr.low_x..dr.high_x + 1 {
                digit_positions.insert(Position { x, y: dr.y }, dr);
            }
        }
    });

    gears
        .par_iter()
        .map(|g| {
            g.surrounding_positions()
                .iter()
                .filter_map(|pos| digit_positions.get(pos))
                .unique()
                .map(|dr| dr.value)
                .collect::<Vec<u32>>()
        })
        .filter(|ds| ds.len() == 2)
        .map(|ds| ds.iter().product::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT), 4361);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT), 467835);
    }
}
