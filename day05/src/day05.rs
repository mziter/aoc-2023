use itertools::Itertools;
use rayon::prelude::*;

//TODO: Lookup time could be possibly improved if we could store the ranges by their
// start positions so we don't try ones that we know won't include the value, but the
// Range lookup checks for range bounds so may be faster as is.
#[derive(Debug)]
struct SeedMap {
    ranges: Vec<Range>,
}

impl SeedMap {
    fn new(ranges: Vec<Range>) -> SeedMap {
        SeedMap { ranges }
    }

    fn lookup(&self, source: u64) -> u64 {
        for range in self.ranges.iter() {
            match range.lookup(source) {
                Some(n) => return n,
                _ => {}
            }
        }
        return source;
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    dest_start: u64,
    source_start: u64,
    len: u64,
}

impl Range {
    fn lookup(&self, source: u64) -> Option<u64> {
        if source >= self.source_start && source < self.source_start + self.len {
            return Some(self.dest_start + (source - self.source_start));
        }
        None
    }
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
        .iter()
        .map(|s| seed_to_soil_info.lookup(*s))
        .map(|s| soil_to_fert_info.lookup(s))
        .map(|s| fert_to_water_info.lookup(s))
        .map(|s| water_to_light_info.lookup(s))
        .map(|s| light_to_temp_info.lookup(s))
        .map(|s| temp_to_humid_info.lookup(s))
        .map(|s| humid_to_locatin_info.lookup(s))
        .min()
        .unwrap()
}

pub fn solve_part_two(input: &str) -> u64 {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    //let seeds_info = parse_seed_range_info(split[0]);
    let seed_to_soil_info = parse_map(split[1]);
    let soil_to_fert_info = parse_map(split[2]);
    let fert_to_water_info = parse_map(split[3]);
    let water_to_light_info = parse_map(split[4]);
    let light_to_temp_info = parse_map(split[5]);
    let temp_to_humid_info = parse_map(split[6]);
    let humid_to_locatin_info = parse_map(split[7]);

    let (_, nums) = split[0].split_once(' ').unwrap();
    nums.split_ascii_whitespace()
        .tuples()
        .flat_map(|(start, len)| {
            let i = start.parse::<u64>().unwrap();
            let j = i + len.parse::<u64>().unwrap();
            i..j
        })
        .map(|s| seed_to_soil_info.lookup(s))
        .map(|s| soil_to_fert_info.lookup(s))
        .map(|s| fert_to_water_info.lookup(s))
        .map(|s| water_to_light_info.lookup(s))
        .map(|s| light_to_temp_info.lookup(s))
        .map(|s| temp_to_humid_info.lookup(s))
        .map(|s| humid_to_locatin_info.lookup(s))
        .min()
        .unwrap()
}

fn parse_range(range_input: &str) -> Range {
    let mut iter = range_input.split_ascii_whitespace();
    Range {
        dest_start: iter.next().unwrap().parse::<u64>().unwrap(),
        source_start: iter.next().unwrap().parse::<u64>().unwrap(),
        len: iter.next().unwrap().parse::<u64>().unwrap(),
    }
}

fn parse_seed_range_info(seed_input: &str) -> Vec<u64> {
    let (_, nums) = seed_input.split_once(' ').unwrap();
    nums.split_ascii_whitespace()
        .tuples()
        .flat_map(|(start, len)| {
            let i = start.parse::<u64>().unwrap();
            let j = i + len.parse::<u64>().unwrap();
            (i..j).collect_vec()
        })
        .collect_vec()
}

fn parse_seed_info(seed_input: &str) -> Vec<u64> {
    let (_, nums) = seed_input.split_once(' ').unwrap();
    nums.split_ascii_whitespace()
        .flat_map(|n| n.parse::<u64>())
        .collect()
}

fn parse_map(map_input: &str) -> SeedMap {
    let ranges: Vec<Range> = map_input
        .lines()
        .skip(1)
        .par_bridge()
        .map(|input| parse_range(input))
        .collect();
    SeedMap::new(ranges)
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
    fn test_range_lookup() {
        let range = Range {
            dest_start: 50,
            source_start: 98,
            len: 2,
        };
        assert_eq!(range.lookup(98), Some(50));
        assert_eq!(range.lookup(99), Some(51));

        assert_eq!(range.lookup(97), None);
        assert_eq!(range.lookup(100), None);
    }

    #[test]
    fn test_seed_to_soil_example() {
        let map = SeedMap {
            ranges: vec![
                Range {
                    dest_start: 50,
                    source_start: 98,
                    len: 2,
                },
                Range {
                    dest_start: 52,
                    source_start: 50,
                    len: 48,
                },
            ],
        };

        assert_eq!(map.lookup(79), 81);
        assert_eq!(map.lookup(14), 14);
        assert_eq!(map.lookup(55), 57);
        assert_eq!(map.lookup(13), 13);
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT), 35);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT), 46);
    }
}
