use std::collections::HashMap;

use crate::Output;

fn is_symbol(byte: u8) -> bool {
    byte != b'.' && !byte.is_ascii_digit()
}

pub fn part1(input: &str) -> Output {
    let grid = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|l| l.len() != 0)
        .collect::<Vec<_>>();

    let mut sum: u32 = 0;
    let mut current: u32 = 0;
    let mut adjacent = false;
    let mut len = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j].is_ascii_digit() {
                current = current * 10 + (grid[i][j] - b'0') as u32;
                if len == 0 && j != 0 {
                    adjacent = is_symbol(grid[i][j - 1]);
                    if i != 0 {
                        adjacent |= is_symbol(grid[i - 1][j - 1]);
                    }
                    if i != grid.len() - 1 {
                        adjacent |= is_symbol(grid[i + 1][j - 1]);
                    }
                }
                if i != 0 {
                    adjacent |= is_symbol(grid[i - 1][j]);
                }
                if i != grid.len() - 1 {
                    adjacent |= is_symbol(grid[i + 1][j]);
                }
                len += 1;
            } else {
                if len != 0 {
                    adjacent |= is_symbol(grid[i][j]);
                    if i != 0 {
                        adjacent |= is_symbol(grid[i - 1][j]);
                    }
                    if i != grid.len() - 1 {
                        adjacent |= is_symbol(grid[i + 1][j]);
                    }
                    if adjacent {
                        sum += current;
                    }
                    current = 0;
                    len = 0;
                    adjacent = false;
                }
            }
        }
        if len != 0 {
            if adjacent {
                sum += current;
            }
            current = 0;
            len = 0;
            adjacent = false;
        }
    }

    sum.into()
}

pub fn part2(input: &str) -> Output {
    let grid = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|l| l.len() != 0)
        .collect::<Vec<_>>();

    let mut current: u32 = 0;
    let mut len = 0;
    let mut symbols = Vec::new();
    let mut gears = HashMap::<_, Vec<_>>::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j].is_ascii_digit() {
                current = current * 10 + (grid[i][j] - b'0') as u32;
                if len == 0 && j != 0 {
                    if grid[i][j - 1] == b'*' {
                        symbols.push((i, j - 1));
                    }
                    if i != 0 && grid[i - 1][j - 1] == b'*' {
                        symbols.push((i - 1, j - 1));
                    }
                    if i != grid.len() - 1 && grid[i + 1][j - 1] == b'*' {
                        symbols.push((i + 1, j - 1));
                    }
                }
                if i != 0 && grid[i - 1][j] == b'*' {
                    symbols.push((i - 1, j));
                }
                if i != grid.len() - 1 && grid[i + 1][j] == b'*' {
                    symbols.push((i + 1, j));
                }
                len += 1;
            } else {
                if len != 0 {
                    if grid[i][j] == b'*' {
                        symbols.push((i, j));
                    }
                    if i != 0 && grid[i - 1][j] == b'*' {
                        symbols.push((i - 1, j));
                    }
                    if i != grid.len() - 1 && grid[i + 1][j] == b'*' {
                        symbols.push((i + 1, j));
                    }
                    for coords in symbols {
                        gears.entry(coords).or_default().push(current);
                    }
                    symbols = vec![];
                    current = 0;
                    len = 0;
                }
            }
        }
        if len != 0 {
            for coords in symbols {
                gears.entry(coords).or_default().push(current);
            }
            symbols = vec![];
            current = 0;
            len = 0;
        }
    }

    gears
        .values()
        .filter_map(|v| (v.len() == 2).then(|| v[0] * v[1]))
        .sum::<u32>()
        .into()
}
