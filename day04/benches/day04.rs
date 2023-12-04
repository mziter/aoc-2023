use aoc_2023::day04::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day_four_part_one() {
    solve_part_one(divan::black_box(include_str!("../input/day04.in")));
}

#[divan::bench]
fn day_four_part_two() {
    solve_part_two(divan::black_box(include_str!("../input/day04.in")));
}
