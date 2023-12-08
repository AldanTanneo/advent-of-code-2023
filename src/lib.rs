#![feature(portable_simd)]

macro_rules! days {
    ($($day:literal),*) => {
        paste::paste! {
            $(mod [<day $day>];)*

            pub const DAYS: &[[fn(&str) -> Output; 2]] = &[
                $([[<day $day>]::part1, [<day $day>]::part2],)*
            ];
        }
    };
}

days!(01, 02, 03, 04, 05, 06, 07, 08);

pub mod utils;

pub type Output = u64;

/*
use std::fmt::Display;

pub struct Output(Box<dyn Display>);

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: ToOwned> From<T> for Output
where
    T::Owned: Display + 'static,
{
    fn from(value: T) -> Self {
        Self(Box::new(value.to_owned()))
    }
}
*/
