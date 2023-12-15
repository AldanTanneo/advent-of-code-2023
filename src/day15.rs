use std::fmt::Debug;

use crate::Output;

fn hash(data: &[u8]) -> u8 {
    data.iter()
        .fold(0, |hash, &b| hash.wrapping_add(b).wrapping_mul(17))
}

pub struct Node<'a> {
    label: &'a [u8],
    data: u8,
    next: Bucket<'a>,
}

pub struct Bucket<'a>(Option<Box<Node<'a>>>);

impl<'a> Bucket<'a> {
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn find_mut<'s>(&'s mut self, key: &[u8]) -> Option<&'s mut u8> {
        let mut s = self.0.as_deref_mut()?;
        loop {
            if s.label == key {
                return Some(&mut s.data);
            } else {
                s = s.next.0.as_deref_mut()?;
            }
        }
    }

    pub fn remove(&mut self, key: &[u8]) -> Option<u8> {
        let mut s = &mut self.0;
        loop {
            if s.as_ref().map(|s| s.label)? == key {
                let value = s.as_ref().map(|s| s.data)?;
                *s = s.as_mut().and_then(|s| s.next.0.take());
                return Some(value);
            } else {
                s = &mut s.as_mut()?.next.0;
            }
        }
    }

    pub fn append(&mut self, key: &'a [u8], value: u8) {
        let node = Node {
            label: key,
            data: value,
            next: Self(self.0.take()),
        };
        *self = Bucket(Some(Box::new(node)))
    }
}

impl<'a> Iterator for &'_ Bucket<'a> {
    type Item = (&'a [u8], u8);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.0.as_deref()?;
        *self = &node.next;
        Some((node.label, node.data))
    }
}

pub struct HashMap<'a> {
    buckets: [Bucket<'a>; 256],
}

impl<'a> HashMap<'a> {
    pub fn new() -> Self {
        const REPEAT: Bucket = Bucket(None);
        Self {
            buckets: [REPEAT; 256],
        }
    }

    pub fn insert(&mut self, key: &'a [u8], value: u8) {
        if let Some(stored) = self.buckets[hash(key) as usize].find_mut(key) {
            *stored = value;
        } else {
            self.buckets[hash(key) as usize].append(key, value);
        }
    }

    pub fn remove(&mut self, key: &[u8]) {
        self.buckets[hash(key) as usize].remove(key);
    }

    pub fn buckets(&self) -> &[Bucket<'a>] {
        &self.buckets
    }
}

impl Debug for HashMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, bucket) in self.buckets().iter().enumerate() {
            if !bucket.is_empty() {
                write!(f, "Box {i}:")?;
                for node in bucket {
                    write!(f, " [{} {}]", std::str::from_utf8(node.0).unwrap(), node.1)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> Output {
    input
        .trim_end()
        .as_bytes()
        .split(|&b| b == b',')
        .map(|s| hash(s) as u64)
        .sum()
}

pub fn part2(input: &str) -> Output {
    let mut hashmap = HashMap::new();
    input
        .trim_end()
        .as_bytes()
        .split(|&b| b == b',')
        .for_each(|entry| match entry {
            [key @ .., b'=', value] => hashmap.insert(key, *value & 0xf),
            [key @ .., b'-'] => hashmap.remove(key),
            _ => panic!("Invalid command"),
        });

    hashmap
        .buckets()
        .iter()
        .enumerate()
        .map(|(box_num, bucket)| {
            let count = bucket.count();
            (box_num + 1)
                * bucket
                    .enumerate()
                    .map(|(slot, (_, focal))| (count - slot) * focal as usize)
                    .sum::<usize>()
        })
        .sum::<usize>() as _
}
