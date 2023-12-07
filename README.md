# Advent Of Code 2023

## About

The advent problems have been solved using Rust. All solutions are in the `src/dayX.rs` files.
There are unit tests for each day which test the examples given in the problem description, as well as the input given in the `inputs/dayX` files.

### Testing & Code Coverage

```bash
cargo llvm-cov nextest --lcov --output-path lcov.info
```

You can use a lcov viewer like Coverage Gutters in VSCode to view the coverage or use the following command to generate a html report.

```bash
genhtml -o coverage lcov.info
```

## [--- Day 1: Trebuchet?! ---](./problems/day1.md)

```bash
cat inputs/day1 | cargo run --bin day1
```

## [--- Day 2: Cube Conundrum ---](./problems/day2.md)

```bash
cat inputs/day2 | cargo run --bin day2
```

## [--- Day 3: Gear Ratios ---](./problems/day3.md)

```bash
cat inputs/day3 | cargo run --bin day3
```

## [--- Day 4: Scratchcards ---](./problems/day4.md)

```bash
cat inputs/day4 | cargo run --bin day4
```

## [--- Day 5: If You Give A Seed A Fertilizer ---](./problems/day5.md)

```bash
cat inputs/day5 | cargo run --bin day5
```

## [--- Day 6: Wait For It ---](./problems/day6.md)

```bash
cat inputs/day6 | cargo run --bin day6
```

## [--- Day 7: Camel Cards ---](./problems/day7.md)

```bash
cat inputs/day7 | cargo run --bin day7
```
