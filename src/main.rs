use std::env::args;

use advent_of_code_2023::*;

fn main() {
    let day = args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .expect("Usage: ./advent-of-code-2023 <n> [input file]");

    let test_file = args().nth(2);

    if day == 0 || day > 25 {
        panic!("Day {day} is not a valid day of Advent of Code.");
    }

    if day > DAYS.len() {
        panic!("Day {day} has not yet come");
    }

    let input = std::fs::read_to_string(
        test_file.unwrap_or_else(|| format!("./inputs/input_{day:02}.txt")),
    )
    .expect("Could not open input file");

    let part1 = DAYS[day - 1][0](&input);
    let part2 = DAYS[day - 1][1](&input);

    println!(
        "{}\x1b[1;31mDay {day}\x1b[0m
\x1b[1;32mPart 1:\x1b[0m {part1}
\x1b[1;32mPart 2:\x1b[0m {part2}",
        include_str!("art.txt"),
    );
}
