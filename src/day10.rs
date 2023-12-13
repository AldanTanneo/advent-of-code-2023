use std::ops::Index;

use itertools::Itertools;

use crate::Output;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Pipe {
    None = b'.',
    Vertical = b'|',
    Horizontal = b'-',
    SouthEast = b'F',
    SouthWest = b'7',
    NorthEast = b'L',
    NorthWest = b'J',
    Invalid = b'\n',
    Start = b'S',
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Clone, Copy)]
struct Grid<'a>(&'a [u8], usize);

impl<'a> Index<usize> for Grid<'a> {
    type Output = [Pipe];

    fn index(&self, index: usize) -> &Self::Output {
        let slice = &self.0[self.1 * index..self.1 * (index + 1) - 1];

        unsafe { std::mem::transmute(slice) }
    }
}

impl<'a> Grid<'a> {
    pub fn new(data: &'a [u8], width: usize) -> Self {
        Self(data, width + 1)
    }

    pub fn len(&self) -> usize {
        self.0.len() / self.1
    }
}

pub fn part1(input: &str) -> Output {
    let input = input.as_bytes();
    let width = input.iter().find_position(|&&b| b == b'\n').unwrap().0;
    let grid = Grid::new(input, width);
    let (i, j) = (0..grid.len())
        .find_map(|i| {
            (0..grid[i].len()).find_map(|j| {
                if grid[i][j] == Pipe::Start {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let paths = {
        use Pipe::*;

        let mut idx = 0;
        let mut res = [((0, 0), Direction::East); 2];

        if i > 0 && matches!(grid[i - 1][j], Vertical | SouthEast | SouthWest) {
            res[idx] = ((i - 1, j), Direction::North);
            idx += 1;
        }
        if i < grid.len() - 1 && matches!(grid[i + 1][j], Vertical | NorthEast | NorthWest) {
            res[idx] = ((i + 1, j), Direction::South);
            idx += 1;
        }
        if j > 0 && matches!(grid[i][j - 1], Horizontal | NorthEast | SouthEast) {
            res[idx] = ((i, j - 1), Direction::West);
            idx += 1;
        }
        if j < grid[i].len() && matches!(grid[i][j + 1], Horizontal | NorthWest | SouthWest) {
            res[idx] = ((i, j + 1), Direction::East);
        }

        res
    };

    std::iter::successors(Some(paths), |p| {
        let mut p = *p;
        for idx in 0..2 {
            if p[idx].0 == p[1 - idx].0 {
                return None;
            }

            let ((i, j), dir) = p[idx];
            p[idx] = match (grid[i][j], dir) {
                (Pipe::Vertical, Direction::North) => ((i - 1, j), dir),
                (Pipe::Vertical, Direction::South) => ((i + 1, j), dir),
                (Pipe::Horizontal, Direction::East) => ((i, j + 1), dir),
                (Pipe::Horizontal, Direction::West) => ((i, j - 1), dir),
                (Pipe::NorthEast, Direction::West) => ((i - 1, j), Direction::North),
                (Pipe::NorthEast, Direction::South) => ((i, j + 1), Direction::East),
                (Pipe::NorthWest, Direction::East) => ((i - 1, j), Direction::North),
                (Pipe::NorthWest, Direction::South) => ((i, j - 1), Direction::West),
                (Pipe::SouthEast, Direction::West) => ((i + 1, j), Direction::South),
                (Pipe::SouthEast, Direction::North) => ((i, j + 1), Direction::East),
                (Pipe::SouthWest, Direction::East) => ((i + 1, j), Direction::South),
                (Pipe::SouthWest, Direction::North) => ((i, j - 1), Direction::West),
                _ => return None,
            };
        }
        Some(p)
    })
    .count() as _
}

pub fn part2(input: &str) -> Output {
    let input = input.as_bytes();
    let width = input.iter().find_position(|&&b| b == b'\n').unwrap().0;
    let grid = Grid::new(input, width);
    let (i, j) = (0..grid.len())
        .find_map(|i| {
            (0..grid[i].len()).find_map(|j| {
                if grid[i][j] == Pipe::Start {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let paths = {
        use Pipe::*;

        let mut idx = 0;
        let mut res = [((0, 0), Direction::East); 2];

        if i > 0 && matches!(grid[i - 1][j], Vertical | SouthEast | SouthWest) {
            res[idx] = ((i - 1, j), Direction::North);
            idx += 1;
        }
        if i < grid.len() - 1 && matches!(grid[i + 1][j], Vertical | NorthEast | NorthWest) {
            res[idx] = ((i + 1, j), Direction::South);
            idx += 1;
        }
        if j > 0 && matches!(grid[i][j - 1], Horizontal | NorthEast | SouthEast) {
            res[idx] = ((i, j - 1), Direction::West);
            idx += 1;
        }
        if j < grid[i].len() && matches!(grid[i][j + 1], Horizontal | NorthWest | SouthWest) {
            res[idx] = ((i, j + 1), Direction::East);
        }

        res
    };

    let s = match (paths[0].1, paths[1].1) {
        (Direction::East, Direction::North) | (Direction::North, Direction::East) => {
            Pipe::NorthEast
        }
        (Direction::West, Direction::North) | (Direction::North, Direction::West) => {
            Pipe::NorthWest
        }
        (Direction::East, Direction::South) | (Direction::South, Direction::East) => {
            Pipe::SouthEast
        }
        (Direction::West, Direction::South) | (Direction::South, Direction::West) => {
            Pipe::SouthWest
        }
        (Direction::West, Direction::East) | (Direction::East, Direction::West) => Pipe::Horizontal,
        (Direction::North, Direction::South) | (Direction::South, Direction::North) => {
            Pipe::Vertical
        }
        _ => Pipe::Invalid,
    };

    let mut visited = vec![vec![]; grid.len()];
    let mut visited2 = vec![vec![]; grid.len()];
    visited2[i].push(j);

    let mut h = [(i, j), (i, j)];
    for idx in 0..2 {
        if matches!(paths[idx].1, Direction::South | Direction::North) {
            h[idx] = paths[idx].0;
        }
    }

    std::iter::successors(Some(paths), |p| {
        let mut p = *p;
        for idx in 0..2 {
            if p[0].0 == p[1].0 {
                return None;
            }

            let ((i, j), dir) = p[idx];

            p[idx] = match (grid[i][j], dir) {
                (Pipe::Vertical, Direction::North) => ((i - 1, j), dir),
                (Pipe::Vertical, Direction::South) => ((i + 1, j), dir),
                (Pipe::Horizontal, Direction::East) => ((i, j + 1), dir),
                (Pipe::Horizontal, Direction::West) => ((i, j - 1), dir),
                (Pipe::NorthEast, Direction::West) => ((i - 1, j), Direction::North),
                (Pipe::NorthEast, Direction::South) => ((i, j + 1), Direction::East),
                (Pipe::NorthWest, Direction::East) => ((i - 1, j), Direction::North),
                (Pipe::NorthWest, Direction::South) => ((i, j - 1), Direction::West),
                (Pipe::SouthEast, Direction::West) => ((i + 1, j), Direction::South),
                (Pipe::SouthEast, Direction::North) => ((i, j + 1), Direction::East),
                (Pipe::SouthWest, Direction::East) => ((i + 1, j), Direction::South),
                (Pipe::SouthWest, Direction::North) => ((i, j - 1), Direction::West),
                _ => return None,
            };
        }
        Some(p)
    })
    .inspect(|p| {
        for idx in 0..2 {
            let ((i, j), _) = p[idx];
            if h[idx].0 != i {
                if h[idx].1 > j {
                    visited[h[idx].0].push(j..=h[idx].1);
                } else {
                    visited[h[idx].0].push(h[idx].1..=j);
                }
                h[idx] = (i, j);
            }
        }
    })
    .last()
    .map(|p| {
        if p[0].0 == p[1].0 {
            let ((i, j), _) = p[0];
            match grid[i][j] {
                Pipe::Vertical => visited[i].push(j..=j),
                Pipe::Horizontal => visited[i].push(h[0].1.min(h[1].1)..=h[0].1.max(h[1].1)),
                Pipe::NorthEast | Pipe::SouthEast => {
                    if h[0].0 == i {
                        visited[i].push(j..=h[0].1);
                    } else {
                        visited[i].push(j..=h[1].1);
                    }
                }
                Pipe::NorthWest | Pipe::SouthWest => {
                    if h[0].0 == i {
                        visited[i].push(h[0].1..=j);
                    } else {
                        visited[i].push(h[1].1..=j);
                    }
                }
                _ => (),
            }
        }
    });

    visited.iter_mut().for_each(|l| {
        l.sort_unstable_by_key(|r| (*r.start(), *r.end()));
    });

    visited
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.array_windows::<2>()
                .fold((false, 0), |(mut is_in, res), [x, y]| {
                    let (mut start, mut end) = (grid[i][*x.start()], grid[i][*x.end()]);
                    if start == Pipe::Start {
                        start = s;
                    } else if end == Pipe::Start {
                        end = s;
                    }
                    if !matches!(
                        (start, end),
                        (Pipe::NorthEast, Pipe::NorthWest) | (Pipe::SouthEast, Pipe::SouthWest)
                    ) {
                        is_in = !is_in;
                    }
                    (
                        is_in,
                        res + if is_in {
                            (y.start() - x.end()).saturating_sub(1)
                        } else {
                            0
                        },
                    )
                })
                .1
        })
        .sum::<usize>() as _
}
