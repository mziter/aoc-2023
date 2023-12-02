use aoc_2023::day02::*;
use aoc_2023::day02_iter::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day_two_part_one() {
    solve_part_one(divan::black_box(include_str!("../input/day02.in")));
}

#[divan::bench]
fn day_two_part_one_with_iter() {
    solve_part_one_with_iterator(divan::black_box(include_str!("../input/day02.in")));
}

#[divan::bench]
fn day_two_part_two() {
    solve_part_two(divan::black_box(include_str!("../input/day02.in")));
}

#[divan::bench]
fn day_two_part_two_with_iter() {
    solve_part_two_with_iterator(divan::black_box(include_str!("../input/day02.in")));
}
