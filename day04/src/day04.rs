use core::panic;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
struct CardResult {
    id: u32,
    winners: u32,
}

pub fn solve_part_one(input: &str) -> u32 {
    input.par_lines().map(score_card).sum()
}

pub fn solve_part_two(input: &str) -> u32 {
    let mut card_counts: [u32; 196] = [1; 196];
    let mut num_winners: [u32; 196] = [0; 196];

    let mut num_cards = 0;
    input.lines().for_each(|card| {
        let result = card_winners(card);
        let idx = result.id as usize;
        num_winners[idx - 1] = result.winners;
        num_cards += 1;
    });

    num_winners
        .iter()
        .enumerate()
        .for_each(|(idx, num_winners)| {
            for i in 1..num_winners + 1 {
                card_counts[idx + (i as usize)] += card_counts[idx]
            }
        });

    card_counts[..num_cards].iter().sum()
}

fn get_score(num_of_winners: u32) -> u32 {
    match num_of_winners {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 8,
        5 => 16,
        6 => 32,
        7 => 64,
        8 => 128,
        9 => 256,
        10 => 512,
        _ => panic!("never expected more than 10 winning numbers"),
    }
}

fn score_card(card: &str) -> u32 {
    let (_, numbers) = card.split_once(':').unwrap();
    let (winners, scratched) = numbers.split_once('|').unwrap();

    let mut winning_numbers: [&str; 10] = [""; 10];
    for (i, winner) in winners.split(' ').filter(|n| !n.is_empty()).enumerate() {
        winning_numbers[i] = winner;
    }

    let mut winners = 0;
    for scratch in scratched.split(' ').filter(|n| !n.is_empty()) {
        if winning_numbers.contains(&scratch) {
            winners += 1;
        }
    }

    get_score(winners)
}

fn card_winners(card: &str) -> CardResult {
    let (game_info, numbers) = card.split_once(':').unwrap();
    let game_number = game_info
        .split_once(' ')
        .expect("game info tuple to be split by space")
        .1
        .trim()
        .parse::<u32>()
        .expect("game info tuple to parse as number");
    let (winners, scratched) = numbers.split_once('|').unwrap();

    let mut winning_numbers: [&str; 10] = [""; 10];
    for (i, winner) in winners.split(' ').filter(|n| !n.is_empty()).enumerate() {
        winning_numbers[i] = winner;
    }

    let mut winners = 0;
    for scratch in scratched.split(' ').filter(|n| !n.is_empty()) {
        if winning_numbers.contains(&scratch) {
            winners += 1;
        }
    }

    CardResult {
        id: game_number,
        winners,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LINE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"#;

    const TEST_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_score_card() {
        assert_eq!(score_card(TEST_LINE), 8);
    }

    #[test]
    fn test_solve_part_one_example() {
        assert_eq!(solve_part_one(TEST_INPUT), 13);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT), 30);
    }
}
