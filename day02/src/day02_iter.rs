use std::str::FromStr;
use std::cmp;
use rayon::prelude::*;

type CubeCount = (CubeColor, u32);

struct SetIterator<'a> {
    data: &'a str,
}

fn set_iterator(data: &str) -> SetIterator {
    SetIterator { data: data.trim() }
}

impl Iterator for SetIterator<'_> {
    type Item = CubeCount;

    fn next(&mut self) -> Option<CubeCount> {
        if self.data.is_empty() {
            return None;
        };
        let read_to_pos = self.data.find(',').unwrap_or(self.data.len());
        let space_pos = self.data.find(' ')?;
        let amount = self.data[..space_pos].parse::<u32>().unwrap();
        let color = CubeColor::from_str(&self.data[space_pos + 1..read_to_pos]).unwrap();

        self.data = if read_to_pos + 2 <= self.data.len() {
            &self.data[read_to_pos + 2..]
        } else {
            ""
        };
        Some((color, amount))
    }
}

#[derive(Debug)]
enum CubeColor {
    Red,
    Blue,
    Green,
}

impl FromStr for CubeColor {
    type Err = ();

    fn from_str(input: &str) -> Result<CubeColor, Self::Err> {
        match input {
            "red" => Ok(CubeColor::Red),
            "blue" => Ok(CubeColor::Blue),
            "green" => Ok(CubeColor::Green),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct GameInfo {
    id: u32,
    max_blue: u32,
    max_red: u32,
    max_green: u32,
}

pub fn solve_part_one_with_iterator(input: &str) -> u32 {
    input
        .par_lines()
        .map(get_game_totals_with_iter)
        .filter(|gc| gc.max_red <= 12 && gc.max_green <= 13 && gc.max_blue <= 14)
        .map(|gc| gc.id)
        .sum()
}

pub fn solve_part_two_with_iterator(input: &str) -> u32 {
    input
        .par_lines()
        .map(get_game_totals_with_iter)
        .map(|gc| gc.max_red * gc.max_green * gc.max_blue)
        .sum()
}

fn get_game_totals_with_iter(line: &str) -> GameInfo {
    let (game_data, sets_data) = line.split_once(':').unwrap();

    let sets: Vec<&str> = sets_data.split(';').collect();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for set_data in sets {
        for set in set_iterator(set_data) {
            match set {
                (CubeColor::Red, amount) => red = cmp::max(red, amount),
                (CubeColor::Blue, amount) => blue = cmp::max(blue, amount),
                (CubeColor::Green, amount) => green = cmp::max(green, amount),
            }
        }
    }

    GameInfo {
        id: game_data[5..].parse::<u32>().unwrap(),
        max_blue: blue,
        max_red: red,
        max_green: green,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

            const EXAMPLE_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn test_solve_part_one_example() {
        assert_eq!(
            solve_part_one_with_iterator(EXAMPLE_INPUT),
            8
        )
    }

    #[test]
    fn test_solve_part_two_example() {
        assert_eq!(
            solve_part_two_with_iterator(EXAMPLE_INPUT),
            2286 
        )
    }

    #[test]
    fn test_game_count() {
        assert_eq!(
            get_game_totals_with_iter(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            GameInfo {
                id: 4,
                max_green: 3,
                max_red: 14,
                max_blue: 15, 
            }
        );

        assert_eq!(
            get_game_totals_with_iter(
                "Game 89: 13 blue, 6 red, 15 green; 5 green, 14 blue, 9 red; 3 green, 15 blue, 5 red; 13 red, 13 green; 18 red, 4 green, 19 blue; 10 green, 10 red, 18 blue"
            ),
            GameInfo {
                id: 89,
                max_green: 15,
                max_red: 18,
                max_blue: 19 
            }
        )
    }

    #[test]
    fn test_game_count_with_iter() {
        assert_eq!(
            get_game_totals_with_iter(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            GameInfo {
                id: 4,
                max_green: 3,
                max_red: 14,
                max_blue: 15, 
            }
        );
    }
}
