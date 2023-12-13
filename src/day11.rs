use crate::{utils::Grid, Output};

fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn calc<const EMPTY: usize>(grid: &str) -> usize {
    let grid = Grid::new(grid.as_bytes());

    let mut galaxies = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'#' {
                galaxies.push([(i, j), (i, j)]);
            }
        }
    }

    for i in 0..grid.len() {
        if grid[i].iter().all(|&x| x == b'.') {
            galaxies.iter_mut().for_each(|g| {
                if g[0].0 > i {
                    g[1].0 += EMPTY - 1;
                }
            });
        }
        if grid.iter().all(|l| l[i] == b'.') {
            galaxies.iter_mut().for_each(|g| {
                if g[0].1 > i {
                    g[1].1 += EMPTY - 1;
                }
            });
        }
    }

    let galaxies = galaxies.as_slice();
    (0..galaxies.len())
        .flat_map(|i| {
            (i + 1..galaxies.len()).map(move |j| manhattan(galaxies[i][1], galaxies[j][1]))
        })
        .sum::<usize>()
}

pub fn part1(input: &str) -> Output {
    calc::<2>(input) as _
}

pub fn part2(input: &str) -> Output {
    calc::<1000000>(input) as _
}
