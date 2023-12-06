use std::collections::VecDeque;

use crate::utils::parse_num;
use crate::Output;

pub fn part1(input: &str) -> Output {
    input
        .lines()
        .filter_map(|line| {
            let (_, game) = line.split_once(':')?;
            let (winning, have) = game.split_once('|')?;

            let mut set = [false; 256];
            winning
                .split_ascii_whitespace()
                .map(parse_num)
                .for_each(|x| set[x as usize] = true);

            let winnings = have
                .split_ascii_whitespace()
                .map(parse_num)
                .filter(|x| set[*x as usize])
                .count();

            winnings.checked_sub(1).map(|x| 1 << x)
        })
        .sum::<u32>()
        .into()
}

pub fn part2(input: &str) -> Output {
    let mut cards = VecDeque::new();
    input
        .lines()
        .filter_map(|line| {
            let res = 1 + cards.pop_front().unwrap_or(0);

            let (_, game) = line.split_once(':')?;
            let (winning, have) = game.split_once('|')?;

            let mut set = [false; 256];
            winning
                .split_ascii_whitespace()
                .map(parse_num)
                .for_each(|x| set[x as usize] = true);

            let winnings = have
                .split_ascii_whitespace()
                .map(parse_num)
                .filter(|x| set[*x as usize])
                .count();

            for i in 0..winnings {
                if i < cards.len() {
                    cards[i] += res;
                } else {
                    cards.push_back(res);
                }
            }

            Some(res)
        })
        .sum::<u32>()
        .into()
}
