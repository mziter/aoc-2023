use aoc_2023::day07::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part_one() {
    solve_part_one(divan::black_box(include_str!("../input/day07.in")));
}

#[divan::bench]
fn part_two() {
    solve_part_two(divan::black_box(include_str!("../input/day07.in")));
}
