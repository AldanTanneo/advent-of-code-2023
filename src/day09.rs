use crate::Output;

const PASCAL: &[i64; 22] = &[
    1, 21, 210, 1330, 5985, 20349, 54264, 116280, 203490, 293930, 352716, 352716, 293930, 203490,
    116280, 54264, 20349, 5985, 1330, 210, 21, 1,
];

pub fn part1(input: &str) -> Output {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .zip(PASCAL)
                .fold((0, 1), |(res, mul), (num, coeff)| {
                    (res + mul * coeff * num, -mul)
                })
                .0
        })
        .sum::<i64>() as _
}

pub fn part2(input: &str) -> Output {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .rev()
                .map(|x| x.parse::<i64>().unwrap())
                .zip(PASCAL)
                .fold((0, 1), |(res, mul), (num, coeff)| {
                    (res + mul * coeff * num, -mul)
                })
                .0
        })
        .sum::<i64>() as _
}
