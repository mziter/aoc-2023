# aoc-2023

Advent of Code 2023 solutions using Rust leaning towards efficieny/performance.

# Benchmarks

Using Divan and running on `AMD Ryzen 7 7800X3D 8-Core Processor`

```
day01                          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ day_one_part_one            33.83 µs      │ 76.64 µs      │ 34.32 µs      │ 36.9 µs       │ 100     │ 100
╰─ day_one_part_two            133.7 µs      │ 193 µs        │ 135.2 µs      │ 139.3 µs      │ 100     │ 100

day02                          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ day_two_part_one            46.17 µs      │ 91.68 µs      │ 46.55 µs      │ 50.34 µs      │ 100     │ 100
├─ day_two_part_one_with_iter  31.69 µs      │ 53.03 µs      │ 32.18 µs      │ 33.97 µs      │ 100     │ 100
├─ day_two_part_two            46.33 µs      │ 64.34 µs      │ 47.1 µs       │ 48.33 µs      │ 100     │ 100
╰─ day_two_part_two_with_iter  32.16 µs      │ 63.94 µs      │ 32.48 µs      │ 33.57 µs      │ 100     │ 100

day03                          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ day_three_part_one          108.8 µs      │ 755.5 µs      │ 127.1 µs      │ 141.6 µs      │ 100     │ 100
╰─ day_three_part_two          240.9 µs      │ 305 µs        │ 259.7 µs      │ 261.9 µs      │ 100     │ 100

day04                          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ day_four_part_one           24.7 µs       │ 796.6 µs      │ 43.99 µs      │ 50.46 µs      │ 100     │ 100
╰─ day_four_part_two           122.8 µs      │ 168.8 µs      │ 123.1 µs      │ 124.6 µs      │ 100     │ 100

day05                          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part_one                    132.2 µs      │ 841.9 µs      │ 141.7 µs      │ 159.7 µs      │ 100     │ 100
╰─ part_two                    7.435 s       │ 9.507 s       │ 8.478 s       │ 8.48 s        │ 100     │ 100

day06                          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part_one                    169.7 ns      │ 1.039 µs      │ 169.7 ns      │ 188.5 ns      │ 100     │ 100
╰─ part_two                    388.7 ns      │ 1.309 µs      │ 389.7 ns      │ 406.7 ns      │ 100     │ 100
```

# Notes

## Day 1

I took a more imperative approach since I believe it would be more performant
when compared to approaches that do lots of replacement or finding every
occurance on a literal or written digit.

## Day 2

I was pleasantly surprised that the iterator implementation was more efficient.
I had a suspicion that all the parsing, spliting, unwrapping, collecting... had
to be fairly inneficient. Even though it would be less performant I will
probably try to use `nom` in a future day just to get experience with it.

## Day 3

It seems like I am leaning more towards implementing iterators for most of these
problems since alternative solutions that are easier will often do a lot more
allocations. Once we have the positions of the numbers and the symbols, we just
check surrounds positions to get the solutions for part one and two.

# Day 4

Fairly straightforward solution to both parts without any really novel approach
IMHO.

# Day 5 (Revisit improving Part Two)

Solution was again, pretty straightforward. The solution to part two is
incredibly slow though. I did have a thought and started to implement a solution
that would merge all the rules together, giving one set of in ranges and out
ranges. Then we could easily try the output ranges in order of lowest to highest
and see when we hit an input range that is in the input. This would save us a
lot of time from blindly trying all possible input. I am not sure if this is the
best or even efficient answer, but I might give it a try for fun. :)

# Day 6

First completed brute force approach. After looking at data, I realized the
output is symetrical, so we only needed to run approximately half. Then I
realized that the success/failure is in order and once you hit the first
"success" every number will be a success until you reach the same point on the
"other side" or descent of the output values. So I implemented a simple binary
search to find the first value that meets the solution and then just use math to
figure out how many more would then succeed.

# Day 7 (Revisit for general refactor)

Approach was to keep things as simple as possible, to mostly keep parsed strings
as strings and not overly convert raw data into objects. Part one logic was pretty
straight forward after implementing sorting logic for hands to prioritize hand type
first and then incrementally the individual cards.

Part two was a tiny bit more difficult as the Joker made the logic to detect different
hands more complicated. This section could definetly use a refactor to simplify!
