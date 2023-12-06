use itertools::Itertools;

pub fn solve_part_one(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let (_, times_str) = lines[0].split_once(':').unwrap();
    let times = times_str
        .split_ascii_whitespace()
        .flat_map(|n| n.parse::<u32>());

    let (_, dists_str) = lines[1].split_once(':').unwrap();
    let dists = dists_str
        .split_ascii_whitespace()
        .flat_map(|n| n.parse::<u32>());

    times
        .zip(dists)
        .map(|(t, d)| {
            let mut sum = 0;
            for i in 0..t + 1 {
                let time_left = t - i;
                let result_dist = i * time_left;
                if result_dist > d {
                    sum += 1
                }
            }
            sum
        })
        .product()
}

pub fn solve_part_two(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let (_, times_str) = lines[0].split_once(':').unwrap();
    let times = times_str.split_ascii_whitespace().collect_vec();
    let time: u64 = times.concat().parse().unwrap();

    let (_, dists_str) = lines[1].split_once(':').unwrap();
    let dists = dists_str.split_ascii_whitespace().collect_vec();
    let dist: u64 = dists.concat().parse().unwrap();

    let mut sum = 0;
    for i in 0..time + 1 {
        let time_left = time - i;
        let result_dist = i * time_left;
        if result_dist > dist {
            sum += 1
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT), 288);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT), 71503);
    }
}
