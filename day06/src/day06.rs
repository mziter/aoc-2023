use itertools::Itertools;

pub fn solve_part_one(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let (_, times_str) = lines[0].split_once(':').unwrap();
    let times = times_str
        .split_ascii_whitespace()
        .flat_map(|n| n.parse::<u64>());

    let (_, dists_str) = lines[1].split_once(':').unwrap();
    let dists = dists_str
        .split_ascii_whitespace()
        .flat_map(|n| n.parse::<u64>());

    times
        .zip(dists)
        .map(|(t, d)| find_solution_binary_search(t, d))
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

    //find_solutions(time, dist)
    find_solution_binary_search(time, dist)
}

// count be sped up if we were to binary search to find first
// instead of iterating
fn find_solutions(time: u64, dist: u64) -> u64 {
    let mut first = 0;
    for i in 0..time + 1 {
        if is_solution(i, time, dist) {
            first = i;
            break;
        }
    }

    get_total(time, first)
}

fn find_solution_binary_search(time: u64, dist: u64) -> u64 {
    let first = binary_search_for_first(time, dist);
    get_total(time, first)
}

fn is_solution(guess: u64, time: u64, dist: u64) -> bool {
    let time_left = time - guess;
    let result_dist = guess * time_left;
    result_dist > dist
}

fn get_total(time: u64, first: u64) -> u64 {
    let time_half = time.div_ceil(2);
    let offset = if time % 2 == 0 { 1 } else { 0 };
    ((time_half - first) * 2) + offset
}

fn binary_search_for_first(time: u64, dist: u64) -> u64 {
    // println!("TIME: {}, DIST: {}", time, dist);
    let mut low: u64 = 1;
    let mut high: u64 = time;
    let mut mid: u64 = high.div_ceil(2);

    loop {
        let last_works = is_solution(mid - 1, time, dist);
        let here_works = is_solution(mid, time, dist);

        if !last_works && here_works {
            return mid;
        }

        // perfect
        if !last_works && here_works {
            return mid;
        }

        // we are too high
        if here_works {
            high = mid;
            mid = (high - low).div_ceil(2);
        }

        // we are too low
        if !here_works {
            low = mid;
            mid = low + ((high - low) / 2);
        }
    }
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
    fn test_binary_search_one_example() {
        assert_eq!(binary_search_for_first(7, 9), 2);
        assert_eq!(binary_search_for_first(15, 40), 4);
        assert_eq!(binary_search_for_first(30, 200), 11);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT), 71503);
    }
}
