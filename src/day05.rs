use crate::utils::parse_num;
use crate::Output;

#[derive(Debug, Clone)]
struct Transposition {
    to: u64,
    from: u64,
    len: u64,
}

#[derive(Debug, Clone)]
struct Mapping(Vec<Transposition>);

impl Mapping {
    fn map(&self, value: u64) -> u64 {
        self.0
            .iter()
            .find_map(|Transposition { to, from, len }| {
                (*from..from + len)
                    .contains(&value)
                    .then(|| value + to - from)
            })
            .unwrap_or(value)
    }

    fn map_intervals(&self, seeds: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        let mut intervals = vec![];
        let mut r = Vec::with_capacity(seeds.len());
        let mut remaining = seeds;
        for Transposition { to, from, len } in &self.0 {
            for (start, len2) in remaining.drain(..) {
                if start + len2 > *from && from + len > start {
                    let (imin, imax) = (start.max(*from), (start + len2).min(from + len));
                    intervals.push((imin + to - from, imax - imin));
                    if start + len2 > from + len {
                        r.push((from + len, start + len2 - from - len));
                    }
                    if start < *from {
                        r.push((start, from - start));
                    }
                } else {
                    r.push((start, len2));
                }
            }
            std::mem::swap(&mut r, &mut remaining);
        }
        intervals.append(&mut remaining);
        intervals
    }
}

fn parse_mappings(data: &str) -> (Vec<u64>, Vec<Mapping>) {
    let data = data.trim_start_matches("seeds: ");
    let (seeds, mut rest) = data.split_once('\n').unwrap();
    let seeds = seeds.split_ascii_whitespace().map(parse_num).collect();

    let mut mappings = vec![];
    while rest.len() != 0 {
        let (_, mut map) = rest.split_once(":\n").unwrap();
        (map, rest) = map.split_once("\n\n").unwrap_or((map, ""));

        mappings.push(Mapping(
            map.lines()
                .map(|l| {
                    let mut it = l.split_ascii_whitespace().map(parse_num);
                    Transposition {
                        to: it.next().unwrap(),
                        from: it.next().unwrap(),
                        len: it.next().unwrap(),
                    }
                })
                .collect(),
        ))
    }

    (seeds, mappings)
}

pub fn part1(input: &str) -> Output {
    let (seeds, mappings) = parse_mappings(input);

    mappings
        .iter()
        .fold(seeds, |seeds, map| {
            seeds.into_iter().map(|v| map.map(v)).collect()
        })
        .into_iter()
        .min()
        .unwrap()
        .into()
}

pub fn part2(input: &str) -> Output {
    use itertools::Itertools;

    let (seeds, mappings) = parse_mappings(input);

    let seeds = seeds.into_iter().tuples().collect::<Vec<_>>();

    let locations = mappings
        .into_iter()
        .fold(seeds, |seeds, map| map.map_intervals(seeds));

    locations
        .iter()
        .min_by_key(|(start, _)| start)
        .unwrap()
        .0
        .into()
}

#[cfg(test)]
#[test]
fn test_map() {
    let map = Mapping(vec![Transposition {
        from: 5,
        to: 10,
        len: 3,
    }]);

    assert_eq!(map.map_intervals(vec![(0, 2)]), vec![(0, 2)]);
    assert_eq!(map.map_intervals(vec![(0, 5)]), vec![(0, 5)]);
    assert_eq!(map.map_intervals(vec![(3, 5)]), vec![(10, 3), (3, 2)]);
    assert_eq!(map.map_intervals(vec![(6, 2)]), vec![(11, 2)]);
    assert_eq!(map.map_intervals(vec![(7, 4)]), vec![(12, 1), (8, 3)]);
    assert_eq!(
        map.map_intervals(vec![(4, 5)]),
        vec![(10, 3), (8, 1), (4, 1)]
    );
    assert_eq!(map.map_intervals(vec![(8, 4)]), vec![(8, 4)]);
    assert_eq!(map.map_intervals(vec![(10, 4)]), vec![(10, 4)]);
    assert_eq!(map.map_intervals(vec![(5, 3)]), vec![(10, 3)]);
}
