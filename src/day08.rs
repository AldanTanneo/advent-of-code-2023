use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashMap as HashMap;

use crate::Output;

type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

pub fn parse_map(input: &str) -> nom::IResult<&str, (Vec<Direction>, Map)> {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::{char, multispace1, newline},
        combinator::{iterator, map},
        multi::many1,
        sequence::{separated_pair, terminated},
    };

    separated_pair(
        terminated(
            many1(alt((
                map(char('L'), |_| Direction::Left),
                map(char('R'), |_| Direction::Right),
            ))),
            newline,
        ),
        multispace1,
        |input| {
            let mut it = iterator(
                input,
                terminated(
                    separated_pair(
                        take(3usize),
                        tag(" = ("),
                        separated_pair(take(3usize), tag(", "), take(3usize)),
                    ),
                    tag(")\n"),
                ),
            );
            let res = it.collect();
            it.finish().map(|(rest, _)| (rest, res))
        },
    )(input)
}

pub fn part1(input: &str) -> Output {
    let (directions, map) = parse_map(input).unwrap().1;
    let mut start = "AAA";

    directions
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(i, dir)| {
            start = match dir {
                Direction::Left => map[start].0,
                Direction::Right => map[start].1,
            };
            if start == "ZZZ" {
                Some(i + 1)
            } else {
                None
            }
        })
        .unwrap() as _
}

pub fn part2(input: &str) -> Output {
    let (directions, map) = parse_map(input).unwrap().1;

    map.par_iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(mut start, _)| {
            directions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(i, dir)| {
                    start = match dir {
                        Direction::Left => &map[start].0,
                        Direction::Right => &map[start].1,
                    };
                    if start.ends_with('Z') {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .reduce(|| 1, num::integer::lcm) as _
}
