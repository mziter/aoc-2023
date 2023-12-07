use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::max;
use std::ops::Range;

#[derive(Debug)]
pub struct SeedMap {
    rules: Vec<MapRule>,
}

impl SeedMap {
    fn translate(&self, s: u64) -> u64 {
        for r in &self.rules {
            if s >= r.in_start && s < r.in_start + r.len {
                let offset = s - r.in_start;
                return r.out_start + offset;
            }
        }
        s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapRule {
    out_start: u64,
    in_start: u64,
    len: u64,
}

impl MapRule {
    fn merge(self, other: MapRule) -> Vec<MapRule> {
        let overlap = self.get_overlap(other);

        todo!()
    }

    /// Get overlap of MapRules out_range and provided MapRules in_range.
    fn get_overlap(&self, other: MapRule) -> Range<u64> {
        let start = max(self.out_start, other.in_start);
        let end = max(self.out_start, other.in_start);
        start..end + 1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapRange {
    in_range: Range<u64>,
    out_range: Range<u64>,
}

pub fn solve_part_one(input: &str) -> u64 {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let seeds_info = parse_seed_info(split[0]);
    let seed_to_soil_info = parse_map(split[1]);
    let soil_to_fert_info = parse_map(split[2]);
    let fert_to_water_info = parse_map(split[3]);
    let water_to_light_info = parse_map(split[4]);
    let light_to_temp_info = parse_map(split[5]);
    let temp_to_humid_info = parse_map(split[6]);
    let humid_to_locatin_info = parse_map(split[7]);

    seeds_info
        .par_iter()
        .map(|s| seed_to_soil_info.translate(*s))
        .map(|s| soil_to_fert_info.translate(s))
        .map(|s| fert_to_water_info.translate(s))
        .map(|s| water_to_light_info.translate(s))
        .map(|s| light_to_temp_info.translate(s))
        .map(|s| temp_to_humid_info.translate(s))
        .map(|s| humid_to_locatin_info.translate(s))
        .min()
        .unwrap()
}

pub fn solve_part_two(input: &str) -> u64 {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let seeds_info = parse_seed_info(split[0]);
    let seed_to_soil_info = parse_map(split[1]);
    let soil_to_fert_info = parse_map(split[2]);
    let fert_to_water_info = parse_map(split[3]);
    let water_to_light_info = parse_map(split[4]);
    let light_to_temp_info = parse_map(split[5]);
    let temp_to_humid_info = parse_map(split[6]);
    let humid_to_locatin_info = parse_map(split[7]);

    seeds_info
        .into_iter()
        .tuples()
        .par_bridge()
        .flat_map(|(start, len)| (start..(start + len)))
        .map(|s| seed_to_soil_info.translate(s))
        .map(|s| soil_to_fert_info.translate(s))
        .map(|s| fert_to_water_info.translate(s))
        .map(|s| water_to_light_info.translate(s))
        .map(|s| light_to_temp_info.translate(s))
        .map(|s| temp_to_humid_info.translate(s))
        .map(|s| humid_to_locatin_info.translate(s))
        .min()
        .unwrap()
}

fn parse_range(range_input: &str) -> MapRule {
    let mut iter = range_input.split_ascii_whitespace();
    MapRule {
        out_start: iter.next().unwrap().parse::<u64>().unwrap(),
        in_start: iter.next().unwrap().parse::<u64>().unwrap(),
        len: iter.next().unwrap().parse::<u64>().unwrap(),
    }
}

fn parse_seed_info(seed_input: &str) -> Vec<u64> {
    let (_, nums) = seed_input.split_once(' ').unwrap();
    nums.split_ascii_whitespace()
        .flat_map(|n| n.parse::<u64>())
        .collect()
}

fn parse_map(map_input: &str) -> SeedMap {
    let rules: Vec<MapRule> = map_input
        .lines()
        .skip(1)
        .par_bridge()
        .map(parse_range)
        .collect();
    SeedMap { rules }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT), 35);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT), 46);
    }
}
