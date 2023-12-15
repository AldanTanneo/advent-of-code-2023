use rayon::{prelude::ParallelIterator, str::ParallelString};
use rustc_hash::FxHashMap;

use crate::{utils, Output};

fn parser_repeat(
    input: &'_ str,
    n: usize,
) -> impl ParallelIterator<Item = (Vec<u8>, Vec<usize>)> + '_ {
    input.par_lines().map(move |l| {
        let (data, idx) = l.split_once(' ').unwrap();
        let idx = idx
            .split(',')
            .map(|x| utils::parse_num(x) as usize)
            .collect::<Vec<_>>();
        let len = idx.len();
        let idx = idx.into_iter().cycle().take(len * n).collect();
        (
            data.bytes()
                .chain(std::iter::once(b'?'))
                .cycle()
                .take(data.len() * n + n - 1)
                .collect(),
            idx,
        )
    })
}

type CacheKey = (u8, u8);

fn descent(
    line: &[u8],
    pos: usize,
    idx: &[usize],
    curr: usize,
    cache: &mut FxHashMap<CacheKey, usize>,
) -> usize {
    let key = (pos as u8, curr as u8);

    if let Some(res) = cache.get(&key) {
        return *res;
    }

    let mut res = 0;
    let data = &line[pos..];
    let to_place = idx[curr];

    for i in 0..data.len().saturating_sub(to_place - 1) {
        let j = i + to_place;

        if data[i..j].iter().all(|&b| b == b'?' || b == b'#') {
            if curr == idx.len() - 1 {
                if data[j..].iter().all(|&b| b != b'#') {
                    res += 1;
                }
            } else if j < data.len() && data[j] != b'#' {
                res += descent(line, pos + j + 1, idx, curr + 1, cache);
            }
        }

        if data[i] == b'#' {
            break;
        }
    }

    cache.insert(key, res);

    res
}

pub fn part1(input: &str) -> Output {
    parser_repeat(input, 1)
        .map(|(line, idx)| {
            let mut cache = FxHashMap::default();
            descent(&line, 0, &idx, 0, &mut cache)
        })
        .sum::<usize>() as _
}

pub fn part2(input: &str) -> Output {
    // let mut cache = FxHashMap::default();
    parser_repeat(input, 5)
        .map(|(line, idx)| {
            let mut cache = FxHashMap::default();
            descent(&line, 0, &idx, 0, &mut cache)
        })
        .sum::<usize>() as _
}
