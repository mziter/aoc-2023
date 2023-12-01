use aoc_2023::day01::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Define a `fibonacci` function and register it for benchmarking.
#[divan::bench]
fn day_one_part_one() {
    solve_part_one(divan::black_box(include_str!("../input/day01.in")));
}

#[divan::bench]
fn day_one_part_two() {
    solve_part_two(divan::black_box(include_str!("../input/day01.in")));
}
