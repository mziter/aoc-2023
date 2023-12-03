use aoc_2023::day03::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day_three_part_one() {
    solve_part_one(divan::black_box(include_str!("../input/day03.in")));
}

#[divan::bench]
fn day_three_part_two() {
    solve_part_two(divan::black_box(include_str!("../input/day03.in")));
}
