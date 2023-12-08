use std::fmt::Debug;

use crate::Output;

macro_rules! gen {
    ($joker:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
        pub enum Card {
            A = 13,
            K = 12,
            Q = 11,
            J = $joker,
            T = 9,
            N9 = 8,
            N8 = 7,
            N7 = 6,
            N6 = 5,
            N5 = 4,
            N4 = 3,
            N3 = 2,
            N2 = 1,
        }

        impl From<u8> for Card {
            fn from(value: u8) -> Self {
                use Card::*;

                match value {
                    b'2' => N2,
                    b'3' => N3,
                    b'4' => N4,
                    b'5' => N5,
                    b'6' => N6,
                    b'7' => N7,
                    b'8' => N8,
                    b'9' => N9,
                    b'T' => T,
                    b'J' => J,
                    b'Q' => Q,
                    b'K' => K,
                    b'A' => A,
                    _ => unreachable!(),
                }
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
        pub enum Kind {
            Five = 6,
            Four = 5,
            Full = 4,
            Three = 3,
            Pairs = 2,
            Two = 1,
            High = 0,
        }

        #[derive(Clone)]
        pub struct Counts([u8; 14]);

        impl Counts {
            pub fn new() -> Self {
                Self([0; 14])
            }

            pub fn add(&mut self, index: Card) {
                self.0[index as usize] += 1;
            }

            pub fn kind(&self) -> Kind {
                use Kind::*;

                let mut tr = [0usize; 6];
                self.0.into_iter().skip(1).for_each(|c| tr[c as usize] += 1);
                let jokers = self.0[0] as usize;
                assert!(jokers <= 5);

                if jokers >= 4 || tr[5 - jokers] == 1 {
                    Five
                } else if jokers >= 3 || tr[4 - jokers] == 1 {
                    Four
                } else if jokers >= 2 || tr[3 - jokers] >= 1 {
                    if jokers <= 1 && tr[2] == 1 + jokers {
                        Full
                    } else {
                        Three
                    }
                } else if tr[2] == 2 - jokers {
                    Pairs
                } else if jokers == 1 || tr[2] == 1 {
                    Two
                } else {
                    High
                }
            }
        }

        impl Debug for Counts {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut formatter = f.debug_list();
                self.0
                    .iter()
                    .fold(&mut formatter, |f, count| f.entry(count))
                    .finish()
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub struct Hand {
            hand: [Card; 5],
            kind: Kind,
            bid: u32,
        }

        impl PartialEq for Hand {
            fn eq(&self, other: &Self) -> bool {
                self.hand == other.hand
            }
        }

        impl PartialOrd for Hand {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                if self.kind != other.kind {
                    self.kind.partial_cmp(&other.kind)
                } else {
                    self.hand.partial_cmp(&other.hand)
                }
            }
        }

        pub fn parse_hand(input: &[u8]) -> nom::IResult<&[u8], Vec<Hand>> {
            use nom::{
                bytes::complete::take,
                character::complete::{char, newline, u32},
                combinator::{eof, map},
                multi::many1,
                sequence::{separated_pair, terminated},
            };

            terminated(
                many1(map(
                    terminated(separated_pair(take(5usize), char(' '), u32), newline),
                    |(hand, bid): (&[u8], _)| {
                        let hand: [Card; 5] = <[u8; 5]>::try_from(hand).unwrap().map(Card::from);
                        let mut counts = Counts::new();
                        for card in hand {
                            counts.add(card);
                        }
                        Hand {
                            hand,
                            kind: counts.kind(),
                            bid,
                        }
                    },
                )),
                eof,
            )(input)
        }
    };
}

pub fn part1(input: &str) -> Output {
    gen!(10);

    let mut hands = parse_hand(input.as_bytes()).unwrap().1;
    hands.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    hands
        .into_iter()
        .enumerate()
        .map(|(i, Hand { bid, .. })| (i + 1) as u64 * bid as u64)
        .sum()
}

pub fn part2(input: &str) -> Output {
    gen!(0);

    let mut hands = parse_hand(input.as_bytes()).unwrap().1;
    hands.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    hands
        .into_iter()
        .enumerate()
        .map(|(i, Hand { bid, .. })| (i + 1) as u64 * bid as u64)
        .sum()
}
