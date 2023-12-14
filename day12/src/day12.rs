struct LineHolder {
    val: String,
}
struct LineGuessIterator<'a> {
    fill_locations: Vec<usize>,
    guess: usize,
    line: &'a mut LineHolder,
}

impl<'a> Iterator for LineGuessIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let num_bits = self.fill_locations.len();
        if self.guess == 1 << self.fill_locations.len() {
            return None;
        }

        for bit_idx in 0..num_bits {
            let bit_val = (self.guess >> bit_idx) & 1;
            let c = if bit_val == 1 { "#" } else { "." };
            let loc = self.fill_locations[bit_idx];
            self.line.val.replace_range(loc..loc + 1, c);
        }

        // get bit values of int
        // use bit values as . or # in fill locations

        self.guess += 1;

        return Some(&self.line.val);
    }
}

impl<'a> LineGuessIterator<'a> {
    fn new(line: String) -> LineGuessIterator<'a> {
        let mut fill_locations = vec![];
        line.chars()
            .enumerate()
            .filter(|(_, c)| c == &'?')
            .for_each(|(i, _)| {
                fill_locations.push(i);
            });

        return LineGuessIterator {
            fill_locations,
            guess: 0,
            line: LineHolder { val: line },
        };
    }
}

pub fn solve_part_one(input: &str) -> u64 {
    todo!()
}

pub fn solve_part_two(input: &str) -> u64 {
    todo!()
}

fn matches_group_numbers(input: &str, broken_counts: &[usize]) -> bool {
    if broken_counts.len() == 0 {
        return true;
    }
    let mut count = 0;
    let mut counting = false;
    for (i, c) in input.chars().enumerate() {
        if c == '#' {
            if !counting {
                counting = true;
            }
            count += 1;
            if (i == input.len() - 1) && &count == broken_counts.first().unwrap() {
                return true;
            }
        } else {
            if counting {
                if &count == broken_counts.first().unwrap() {
                    return true && matches_group_numbers(&input[i + 1..], &broken_counts[1..]);
                }
                return false;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EXAMPLE: &str = r#""#;

    #[test]
    fn test_matches_group_numbers() {
        assert!(matches_group_numbers(".#.", &[1]));
        assert!(matches_group_numbers("#.#.###", &[1, 1, 3]));
        assert!(matches_group_numbers(".#.###.#.######", &[1, 3, 1, 6]));
        assert!(matches_group_numbers("####.#...#...", &[4, 1, 1]));
        assert!(matches_group_numbers("#....######..#####.", &[1, 6, 5]));
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(TEST_EXAMPLE), 374);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_EXAMPLE), 1030);
    }
}
