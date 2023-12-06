use crate::Output;

use itertools::{Itertools, MinMaxResult};

pub fn part1(input: &str) -> Output {
    let res: u32 = input
        .lines()
        .filter_map(|line| {
            let mut digits = line.chars().filter(char::is_ascii_digit);

            let first = digits.next()?.to_digit(10)?;
            let last = digits.last().and_then(|s| s.to_digit(10)).unwrap_or(first);

            Some(first * 10 + last)
        })
        .sum();

    res.into()
}

pub fn part2(input: &str) -> Output {
    let digits = &[
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let res: u32 = input
        .lines()
        .filter_map(|line| {
            match digits
                .iter()
                .flat_map(|(s, v)| [line.find(s).map(|i| (i, v)), line.rfind(s).map(|i| (i, v))])
                .flatten()
                .minmax_by_key(|(i, _)| *i)
            {
                MinMaxResult::NoElements => None,
                MinMaxResult::OneElement((_, v)) => Some(v * 11),
                MinMaxResult::MinMax((_, first), (_, last)) => Some(first * 10 + last),
            }
        })
        .sum();

    res.into()
}
