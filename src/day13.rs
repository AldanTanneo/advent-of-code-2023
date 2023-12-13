use rayon::{iter::ParallelBridge, prelude::ParallelIterator};

use crate::{utils::Grid, Output};

pub fn part1(input: &str) -> Output {
    input
        .split_inclusive("\n\n")
        .map(|g| {
            let grid = Grid::new(g.as_bytes());

            for i in 1..grid.len() {
                if (0..i)
                    .rev()
                    .zip(i..grid.len())
                    .all(|(x, y)| grid[x] == grid[y])
                {
                    return 100 * i;
                }
            }

            for j in 1..grid.width() {
                if (0..j)
                    .rev()
                    .zip(j..grid.width())
                    .all(|(x, y)| grid.iter().all(|l| l[x] == l[y]))
                {
                    return j;
                }
            }

            unreachable!()
        })
        .sum::<usize>() as _
}

pub fn part2(input: &str) -> Output {
    input
        .split_inclusive("\n\n")
        .par_bridge()
        .map(|g| {
            let grid = Grid::new(g.as_bytes());

            for i in 1..grid.len() {
                if (0..i)
                    .rev()
                    .zip(i..grid.len())
                    .map(|(x, y)| {
                        grid[x]
                            .iter()
                            .zip(&grid[y])
                            .filter(|(cx, cy)| cx != cy)
                            .count()
                    })
                    .sum::<usize>()
                    == 1
                {
                    return 100 * i;
                }
            }

            for j in 1..grid.width() {
                if (0..j)
                    .rev()
                    .zip(j..grid.width())
                    .map(|(x, y)| grid.iter().filter(|l| l[x] != l[y]).count())
                    .sum::<usize>()
                    == 1
                {
                    return j;
                }
            }

            unreachable!()
        })
        .sum::<usize>() as _
}
