use rustc_hash::FxHashMap;

use crate::{
    utils::{Grid, VecGrid},
    Output,
};

fn bearing_after_north_tilt(grid: &Grid<'_>) -> usize {
    let mut res = 0;
    for j in 0..grid.width() {
        let mut cnt = 0;
        let mut start = 0;
        for i in 0..grid.len() {
            match grid[i][j] {
                b'O' => cnt += 1,
                b'.' => (),
                b'#' => {
                    let up = grid.len() - start;
                    let down = up - cnt;
                    let sum = (up * (up + 1) - down * (down + 1)) >> 1;
                    res += sum;

                    cnt = 0;
                    start = i + 1;
                }
                _ => unreachable!(),
            }
        }
        if cnt != 0 {
            let up = grid.len() - start;
            let down = up - cnt;
            let sum = (up * (up + 1) - down * (down + 1)) >> 1;
            res += sum;
        }
    }
    res
}

fn bearing(grid: &VecGrid) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, l)| (i + 1) * l.iter().filter(|&&b| b == b'O').count())
        .sum()
}

fn tilt<T>(grid: &mut T, f: impl Fn(&mut T, usize, usize) -> &mut u8, w: usize, l: usize) {
    for j in 0..w {
        let mut cnt = 0;
        let mut start = 0;
        for i in 0..l {
            match *f(grid, i, j) {
                b'O' => {
                    *f(grid, i, j) = b'.';
                    cnt += 1
                }
                b'.' => (),
                b'#' => {
                    for k in start..start + cnt {
                        *f(grid, k, j) = b'O'
                    }

                    cnt = 0;
                    start = i + 1;
                }
                _ => unreachable!(),
            }
        }
        if cnt != 0 {
            for k in start..start + cnt {
                *f(grid, k, j) = b'O'
            }
        }
    }
}

fn tilt_north(grid: &mut VecGrid) {
    let w = grid.width();
    let l = grid.len();
    tilt(grid, move |g, i, j| &mut g[i][j], w, l)
}

fn tilt_south(grid: &mut VecGrid) {
    let w = grid.width();
    let l = grid.len();
    tilt(grid, move |g, i, j| &mut g[l - i - 1][j], w, l)
}

fn tilt_west(grid: &mut VecGrid) {
    let w = grid.width();
    let l = grid.len();
    tilt(grid, move |g, i, j| &mut g[j][i], l, w)
}

fn tilt_east(grid: &mut VecGrid) {
    let w = grid.width();
    let l = grid.len();
    tilt(grid, move |g, i, j| &mut g[j][w - i - 1], l, w)
}

pub fn part1(input: &str) -> Output {
    let grid = Grid::new(input.as_bytes());

    bearing_after_north_tilt(&grid) as _
}

const CYCLES: usize = 1000_000_000;

pub fn part2(input: &str) -> Output {
    let mut grid = VecGrid::new(input.as_bytes().to_vec());

    let mut set = FxHashMap::default();

    for i in 0..CYCLES {
        if let Some(it) = set.get(&grid) {
            let target = it + (CYCLES - it) % (i - it);
            let grid = set.iter().find(|(_, i)| **i == target).unwrap().0;
            return bearing(grid) as _;
        }

        set.insert(grid.clone(), i);

        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
    }

    bearing(&grid) as _
}
