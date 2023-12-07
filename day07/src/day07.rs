use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand<'a> {
    hand_type: HandType,
    cards: &'a str,
    bid_amount: u32,
    joker_rules: bool,
}

impl<'a> Hand<'a> {
    fn new(cards: &'a str, bid_amount: u32, joker_rules: bool) -> Hand<'a> {
        Hand {
            hand_type: Self::detect_hand_type(cards, joker_rules),
            cards,
            bid_amount,
            joker_rules,
        }
    }

    //TODO: The sort and dedup here are pretty costly in time and allocations.
    //      This would be a good place to optimize if desired.
    fn detect_hand_type(cards: &'a str, joker_rules: bool) -> HandType {
        let mut card_counts: Vec<(usize, char)> = cards
            .chars()
            .sorted_unstable()
            .dedup_with_count()
            .sorted_by(|(a_count, _), (b_count, _)| a_count.cmp(b_count))
            .rev()
            .collect();

        let joker_count = if joker_rules {
            if let Some((count, _)) = card_counts.iter().find(|(_, card)| card == &'J') {
                *count
            } else {
                0
            }
        } else {
            0
        };
        if joker_rules {
            card_counts.retain(|(_, c)| c != &'J');
        }
        if joker_count == 5 {
            return HandType::FiveOfKind;
        }

        let first_count = card_counts[0].0;

        if first_count == 5 || first_count + joker_count == 5 {
            return HandType::FiveOfKind;
        }
        let second_count = card_counts[1].0;
        if first_count == 4 || first_count + joker_count == 4 {
            return HandType::FourOfKind;
        }
        if ((first_count == 3 || first_count + joker_count == 3) && second_count == 2)
            || (first_count == 3 && (second_count == 2 || second_count + joker_count == 2))
        {
            return HandType::FullHouse;
        }
        if first_count == 3 || first_count + joker_count == 3 {
            return HandType::ThreeOfKind;
        }
        if ((first_count == 2 || first_count + joker_count == 2) && second_count == 2)
            || (first_count == 2 && (second_count == 2 || second_count + joker_count == 2))
        {
            return HandType::TwoPair;
        }
        if first_count == 2 || first_count + joker_count == 2 {
            return HandType::OnePair;
        }
        HandType::HighCard
    }
}

const CARD_ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CARD_JOKER_ORDER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_cmp = self.hand_type.cmp(&other.hand_type);
        if hand_cmp == Ordering::Equal {
            let hand_tuples = self.cards.chars().zip(other.cards.chars());

            for (self_card, other_card) in hand_tuples {
                if self_card == other_card {
                    continue;
                }

                let ord = if self.joker_rules {
                    CARD_JOKER_ORDER
                } else {
                    CARD_ORDER
                };

                let a_pos = ord.iter().position(|c| c == &self_card);
                let b_pos = ord.iter().position(|c| c == &other_card);

                if a_pos > b_pos {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
        }
        hand_cmp
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Eq for Hand<'_> {}

pub fn solve_part_one(input: &str) -> u64 {
    //TODO:  will it help to parallelize the parsing?
    input
        .lines()
        .map(|l| parse_line(l, false))
        .sorted_unstable()
        .rev()
        .enumerate()
        .map(|(i, h)| ((i + 1) as u64) * h.bid_amount as u64)
        .sum()
}

pub fn solve_part_two(input: &str) -> u64 {
    input
        .lines()
        .map(|l| parse_line(l, true))
        .sorted_unstable()
        .rev()
        .enumerate()
        .map(|(i, h)| ((i + 1) as u64) * h.bid_amount as u64)
        .sum()
}

fn parse_line(line: &str, joker_rules: bool) -> Hand {
    let (cards, bid) = line.split_once(' ').unwrap();
    let bid_amount = bid.parse::<u32>().unwrap();
    Hand::new(cards, bid_amount, joker_rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    const HAND_A: Hand = Hand {
        hand_type: HandType::OnePair,
        cards: "32T3K",
        bid_amount: 765,
        joker_rules: false,
    };

    const HAND_B: Hand = Hand {
        hand_type: HandType::ThreeOfKind,
        cards: "T55J5",
        bid_amount: 684,
        joker_rules: false,
    };

    const HAND_C: Hand = Hand {
        hand_type: HandType::TwoPair,
        cards: "KK677",
        bid_amount: 28,
        joker_rules: false,
    };

    const HAND_D: Hand = Hand {
        hand_type: HandType::TwoPair,
        cards: "KTJJT",
        bid_amount: 220,
        joker_rules: false,
    };

    const HAND_E: Hand = Hand {
        hand_type: HandType::ThreeOfKind,
        cards: "QQQJA",
        bid_amount: 483,
        joker_rules: false,
    };

    #[test]
    fn test_hand_types() {
        assert_eq!(
            HandType::OnePair,
            Hand::detect_hand_type(HAND_A.cards, false)
        );
        assert_eq!(
            HandType::ThreeOfKind,
            Hand::detect_hand_type(HAND_B.cards, false)
        );
        assert_eq!(
            HandType::TwoPair,
            Hand::detect_hand_type(HAND_C.cards, false)
        );
        assert_eq!(
            HandType::TwoPair,
            Hand::detect_hand_type(HAND_D.cards, false)
        );
        assert_eq!(
            HandType::ThreeOfKind,
            Hand::detect_hand_type(HAND_E.cards, false)
        );

        assert_eq!(HandType::HighCard, Hand::detect_hand_type("AKQJT", false));
        assert_eq!(HandType::OnePair, Hand::detect_hand_type("AKQKT", false));
        assert_eq!(HandType::FullHouse, Hand::detect_hand_type("AKAKA", false));
    }

    #[test]
    fn test_hand_ord() {
        let mut input = [HAND_A, HAND_B, HAND_C, HAND_D, HAND_E];
        input.sort();
        assert_eq!(input, [HAND_E, HAND_B, HAND_C, HAND_D, HAND_A]);
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT), 6440);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT), 5905);
    }
}
