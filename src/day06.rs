use crate::Output;

pub fn part1(input: &str) -> Output {
    use std::simd::{f32x4, SimdFloat, StdFloat};

    #[inline(always)]
    fn stoi(inp: &mut impl Iterator<Item = u8>, first: u8) -> u64 {
        inp.take_while(|c| c.is_ascii_digit())
            .fold((first - b'0') as u64, |acc, curr| {
                acc * 10 + (curr - b'0') as u64
            })
    }

    let mut input = input.bytes().skip(10);

    let mut races = [0.0; 8];
    let mut races_insert = &mut races[..];

    while let Some(first) = input.find(|&c| c.is_ascii_digit()) {
        let num = stoi(&mut input, first);
        *races_insert.first_mut().unwrap() = num as f32;
        races_insert = &mut races_insert[1..];
    }

    let t = f32x4::from_array(races[..4].try_into().unwrap());
    let d = f32x4::from_array(races[4..].try_into().unwrap());

    let four = f32x4::splat(4.0);
    let two = f32x4::splat(2.0);
    let one = f32x4::splat(1.0);
    let half = f32x4::splat(0.5);

    let delta = t * t - four * (d + one);
    let x = (t - delta.sqrt()) * half;
    let res = (t + one - x.ceil() * two).reduce_product();
    res as _
}

pub fn part2(input: &str) -> Output {
    let mut t: u64 = 0;
    let mut it = input.bytes().skip(10);
    for b in it.by_ref() {
        if b == b'\n' {
            break;
        }
        if b != b' ' {
            t = t * 10 + (b & 0xf) as u64
        }
    }
    let mut d: u64 = 0;
    for b in it.skip(10) {
        if b == b'\n' {
            break;
        }
        if b != b' ' {
            d = d * 10 + (b & 0xf) as u64
        }
    }

    let delta = (t * t - 4 * (d + 1)) as f64;
    let x = (t as f64 - delta.sqrt()) * 0.5;
    assert!(x.is_sign_positive());
    t + 1 - x.ceil() as u64 * 2
}
