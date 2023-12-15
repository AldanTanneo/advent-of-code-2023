use std::fmt::Write;

pub fn parse_num(x: &str) -> u64 {
    x.bytes().fold(0, |acc, x| acc * 10 + (x & 0xf) as u64)
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Grid<'a>(&'a [u8], usize);

impl<'a> Grid<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let width = data
            .iter()
            .enumerate()
            .find(|&(_, b)| *b == b'\n')
            .unwrap()
            .0;

        Self(data, width + 1)
    }

    pub fn len(&self) -> usize {
        self.0.len() / self.1
    }

    pub fn width(&self) -> usize {
        self.1 - 1
    }

    pub fn iter(&'a self) -> impl DoubleEndedIterator<Item = &'a [u8]> {
        (0..self.len()).map(move |i| &self[i])
    }
}

impl<'a> std::ops::Index<usize> for Grid<'a> {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[self.1 * index..self.1 * (index + 1) - 1]
    }
}

impl<'a> std::iter::IntoIterator for &'a Grid<'a> {
    type IntoIter = impl DoubleEndedIterator<Item = &'a [u8]>;
    type Item = &'a [u8];

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl std::fmt::Debug for Grid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self {
            for c in l {
                f.write_char(*c as char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct VecGrid(Vec<u8>, usize);

impl VecGrid {
    pub fn new(data: Vec<u8>) -> Self {
        let width = data
            .iter()
            .enumerate()
            .find(|&(_, b)| *b == b'\n')
            .unwrap()
            .0;

        Self(data, width + 1)
    }

    pub fn len(&self) -> usize {
        self.0.len() / self.1
    }

    pub fn width(&self) -> usize {
        self.1 - 1
    }

    pub fn grid(&self) -> Grid<'_> {
        Grid(&self.0, self.1)
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &[u8]> {
        (0..self.len()).map(move |i| &self[i])
    }

    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut [u8]> {
        let w = self.width();
        self.0.chunks_exact_mut(self.1).map(move |s| &mut s[..w])
    }
}

impl std::ops::Index<usize> for VecGrid {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[self.1 * index..self.1 * (index + 1) - 1]
    }
}

impl std::ops::IndexMut<usize> for VecGrid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[self.1 * index..self.1 * (index + 1) - 1]
    }
}

impl<'a> std::iter::IntoIterator for &'a VecGrid {
    type IntoIter = impl DoubleEndedIterator<Item = &'a [u8]>;
    type Item = &'a [u8];

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut VecGrid {
    type IntoIter = impl DoubleEndedIterator<Item = &'a mut [u8]>;
    type Item = &'a mut [u8];

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl std::fmt::Debug for VecGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid().fmt(f)
    }
}
