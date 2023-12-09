use core::panic;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Location<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

pub fn solve_part_one(input: &str) -> u64 {
    let (directions, location_section) = input.split_once("\n\n").unwrap();

    let mut locations = HashMap::new();
    location_section.lines().map(parse_location).for_each(|l| {
        locations.insert(l.name, l);
    });

    let mut location = "AAA";
    let mut turns_taken = 0;
    let mut directions_iter = directions.chars().cycle();
    while location != "ZZZ" {
        let location_info = locations.get(location).unwrap();
        let d = directions_iter.next().unwrap();

        match d {
            'L' => {
                location = location_info.left;
            }
            'R' => {
                location = location_info.right;
            }
            _ => panic!("expected to only encounter L and R"),
        }

        turns_taken += 1;
    }

    turns_taken
}

pub fn find_cycle_length<'a>(
    mut location: &'a str,
    directions: &str,
    lookup: &HashMap<&str, Location<'a>>,
) -> u64 {
    let mut turns = 0;
    let mut directions_iter = directions.chars().cycle();
    while !location.ends_with('Z') {
        let d = directions_iter.next().unwrap();
        let location_info = lookup.get(location).unwrap();
        match d {
            'L' => location = location_info.left,
            'R' => location = location_info.right,
            _ => panic!("expected to only encounter L and R"),
        }
        turns += 1;
    }
    turns
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

pub fn lcm(m: u64, n: u64) -> u64 {
    (m * n) / gcd(m, n)
}

pub fn solve_part_two(input: &str) -> u64 {
    let (directions, location_section) = input.split_once("\n\n").unwrap();

    let mut locations = HashMap::new();
    let mut current_locations = Vec::with_capacity(100);
    location_section.lines().map(parse_location).for_each(|l| {
        let name = l.name;
        locations.insert(name, l);
        if name.ends_with('A') {
            current_locations.push(name);
        }
    });

    let lcm: u64 = current_locations
        .par_iter()
        .map(|loc| find_cycle_length(loc, directions, &locations))
        .reduce(|| 1, lcm);

    lcm
}

fn parse_location(line: &str) -> Location {
    let (name, turn_section) = line.split_once(" = ").unwrap();
    let (left_part, right_part) = turn_section.split_once(", ").unwrap();

    Location {
        name,
        left: &left_part[1..],
        right: &right_part[..3],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EXAMPLE_ONE: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    const TEST_EXAMPLE_TWO: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    const TEST_EXAMPLE_THREE: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn test_solve_part_one_example_one() {
        assert_eq!(solve_part_one(TEST_EXAMPLE_ONE), 2);
    }

    #[test]
    fn test_solve_part_one_example_two() {
        assert_eq!(solve_part_one(TEST_EXAMPLE_TWO), 6);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_EXAMPLE_THREE), 6);
    }
}
