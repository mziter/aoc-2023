use std::cmp;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct GameInfo {
    id: u32,
    max_blue: u32,
    max_red: u32,
    max_green: u32,
}

pub fn solve_part_one(input: &str) -> u32 {
    input
        .par_lines()
        .map(get_game_totals)
        .filter(|gc| gc.max_red <= 12 && gc.max_green <= 13 && gc.max_blue <= 14)
        .map(|gc| gc.id)
        .sum()
}

pub fn solve_part_two(input: &str) -> u32 {
    input
        .par_lines()
        .map(get_game_totals)
        .map(|gc| gc.max_red * gc.max_green * gc.max_blue)
        .sum()
}

fn get_game_totals(line: &str) -> GameInfo {
    let tokens: Vec<&str> = line.split([':', ',', ';']).collect();
    let id = tokens[0][5..tokens[0].len()].parse::<u32>().unwrap();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for token in tokens[1..].iter() {
        let info: Vec<&str> = token[1..].split_whitespace().collect();
        let count = info[0].parse::<u32>().unwrap();
        match info[1].chars().next().unwrap() {
            'b' => blue = cmp::max(count, blue),
            'g' => green = cmp::max(count, green),
            'r' => red = cmp::max(count, red),
            _ => panic!("encountered unexpected color character"),
        }
    }

    GameInfo {
        id,
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
            solve_part_one(EXAMPLE_INPUT),
            8
        )
    }

    #[test]
    fn test_solve_part_two_example() {
        assert_eq!(
            solve_part_two(EXAMPLE_INPUT),
            2286 
        )
    }

    #[test]
    fn test_game_count() {
        assert_eq!(
            get_game_totals(
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
            get_game_totals(
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
            get_game_totals(
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
