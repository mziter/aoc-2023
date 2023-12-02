# aoc-2023

Advent of Code 2023 solutions using Rust leaning towards efficieny/performance.

## Benchmarks

Running on `AMD Ryzen 7 7800X3D 8-Core Processor`

```
day01                fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ day_one_part_one  33.83 µs      │ 76.64 µs      │ 34.32 µs      │ 36.9 µs       │ 100     │ 100
╰─ day_one_part_two  133.7 µs      │ 193 µs        │ 135.2 µs      │ 139.3 µs      │ 100     │ 100

day02                          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ day_two_part_one            46.17 µs      │ 91.68 µs      │ 46.55 µs      │ 50.34 µs      │ 100     │ 100
├─ day_two_part_one_with_iter  31.69 µs      │ 53.03 µs      │ 32.18 µs      │ 33.97 µs      │ 100     │ 100
├─ day_two_part_two            46.33 µs      │ 64.34 µs      │ 47.1 µs       │ 48.33 µs      │ 100     │ 100
╰─ day_two_part_two_with_iter  32.16 µs      │ 63.94 µs      │ 32.48 µs      │ 33.57 µs      │ 100     │ 100
```

## Day 01 Notes

I took a more imperative approach since I believe it would be more performant
when compared to approaches that do lots of replacement or finding every
occurance on a literal or written digit.

## Day 02 Notes

I was pleasantly surprised that the iterator implementation was more efficient.
I had a suspicion that all the parsing, spliting, unwrapping, collecting... had
to be fairly inneficient. I might consider using `nom` or some other parsing
library to see if direct parsing would be even more efficient, though I wouldn't
think so.
