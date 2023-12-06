use crate::utils::parse_num;
use crate::Output;

pub fn part1(input: &str) -> Output {
    let (time, distance) = input.split_once('\n').unwrap();

    time[10..]
        .split_ascii_whitespace()
        .map(|t| parse_num(t))
        .zip(
            distance[10..]
                .split_ascii_whitespace()
                .map(|t| parse_num(t)),
        )
        .map(|(t, d)| {
            let delta = t * t - 4 * (d + 1);
            let x = (t as f64 - (delta as f64).sqrt()) / 2.0;
            t + 1 - x.ceil() as u64 * 2
        })
        .product::<u64>()
        .into()
}

pub fn part2(input: &str) -> Output {
    let (time, distance) = input.split_once('\n').unwrap();
    let t = time[10..]
        .bytes()
        .filter(|x| x.is_ascii_digit())
        .fold(0, |acc, x| acc * 10 + (x & 0xf) as usize);
    let d = distance[10..]
        .bytes()
        .filter(|x| x.is_ascii_digit())
        .fold(0, |acc, x| acc * 10 + (x & 0xf) as usize);

    let delta = t * t - 4 * (d + 1);
    let x = (t as f64 - (delta as f64).sqrt()) / 2.0;
    let res = t + 1 - x.ceil() as usize * 2;

    res.into()
}
