use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;

impl LineGuessIterator {
    fn new(line: &str) -> LineGuessIterator {
        let mut fill_locations = vec![];
        line.chars()
            .enumerate()
            .filter(|(_, c)| c == &'?')
            .for_each(|(i, _)| {
                fill_locations.push(i);
            });

        LineGuessIterator {
            fill_locations,
            guess: 0,
            line: line.to_owned(),
        }
    }
}

struct LineGuessIterator {
    fill_locations: Vec<usize>,
    guess: usize,
    line: String,
}

impl Iterator for LineGuessIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let num_bits = self.fill_locations.len();
        if self.guess == 1 << self.fill_locations.len() {
            return None;
        }

        for bit_idx in 0..num_bits {
            let bit_val = (self.guess >> bit_idx) & 1;
            let c = if bit_val == 1 { "#" } else { "." };
            let loc = self.fill_locations[bit_idx];
            self.line.replace_range(loc..loc + 1, c);
        }

        // get bit values of int
        // use bit values as . or # in fill locations

        self.guess += 1;

        Some(self.line[..].to_string())
    }
}

pub fn solve_part_one(input: &str) -> usize {
    input
        .par_lines()
        .map(|l| {
            let mut matches = 0;
            let (record, broken_counts) = l.split_once(' ').unwrap();
            for guess in LineGuessIterator::new(record) {
                if matches_group_numbers(
                    &guess,
                    &broken_counts
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>(),
                ) {
                    matches += 1;
                }
            }
            matches
        })
        .sum()
}

pub fn solve_part_two(input: &str) -> u64 {
    input
        .par_lines()
        .map(|l| {
            let mut matches = 0;
            let (record, broken_counts) = l.split_once(' ').unwrap();
            let mut expanded = String::with_capacity((record.len() * 5) + 4);
            expanded.push_str(record);
            for _ in 0..5 {
                expanded.push('?');
                expanded.push_str(record);
            }
            for guess in LineGuessIterator::new(&expanded) {
                if matches_group_numbers(
                    &guess,
                    &broken_counts
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                        .repeat(5),
                ) {
                    matches += 1;
                }
            }
            matches
        })
        .sum()
}

fn matches_group_numbers(input: &str, broken_counts: &[usize]) -> bool {
    if broken_counts.is_empty() {
        return !input.contains('#');
    }
    let mut count = 0;
    let mut counting = false;
    for (i, c) in input.chars().enumerate() {
        if c == '#' {
            if !counting {
                counting = true;
            }
            count += 1;
            if (i == input.len() - 1)
                && &count == broken_counts.first().unwrap()
                && broken_counts.len() == 1
            {
                return true;
            }
            continue;
        }
        if counting {
            if &count == broken_counts.first().unwrap() {
                return matches_group_numbers(&input[i + 1..], &broken_counts[1..]);
            }
            return false;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EXAMPLE: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn test_matches_group_numbers() {
        assert!(matches_group_numbers(".#.", &[1]));
        assert!(matches_group_numbers("#.#.###", &[1, 1, 3]));
        assert!(matches_group_numbers(".#.###.#.######", &[1, 3, 1, 6]));
        assert!(matches_group_numbers("####.#...#...", &[4, 1, 1]));
        assert!(matches_group_numbers("#....######..#####.", &[1, 6, 5]));
    }

    #[test]
    fn test_line_guess_iterator() {
        let input = "???.###";
        let iter = LineGuessIterator::new(input);
        let items = iter.collect::<Vec<String>>();
        println!("{:?}", items);
        assert_eq!(8, items.len());
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
