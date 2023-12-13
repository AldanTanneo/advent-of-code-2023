#![feature(portable_simd)]
#![feature(array_chunks)]
#![feature(array_windows)]
#![feature(impl_trait_in_assoc_type)]

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

days!(01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13);

pub mod utils;

pub type Output = u64;
