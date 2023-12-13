use crate::Output;

const PASCAL: &[&[i64]; 22] = &[
    &[1],
    &[1, 1],
    &[1, 2, 1],
    &[1, 3, 3, 1],
    &[1, 4, 6, 4, 1],
    &[1, 5, 10, 10, 5, 1],
    &[1, 6, 15, 20, 15, 6, 1],
    &[1, 7, 21, 35, 35, 21, 7, 1],
    &[1, 8, 28, 56, 70, 56, 28, 8, 1],
    &[1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
    &[1, 10, 45, 120, 210, 252, 210, 120, 45, 10, 1],
    &[1, 11, 55, 165, 330, 462, 462, 330, 165, 55, 11, 1],
    &[1, 12, 66, 220, 495, 792, 924, 792, 495, 220, 66, 12, 1],
    &[
        1, 13, 78, 286, 715, 1287, 1716, 1716, 1287, 715, 286, 78, 13, 1,
    ],
    &[
        1, 14, 91, 364, 1001, 2002, 3003, 3432, 3003, 2002, 1001, 364, 91, 14, 1,
    ],
    &[
        1, 15, 105, 455, 1365, 3003, 5005, 6435, 6435, 5005, 3003, 1365, 455, 105, 15, 1,
    ],
    &[
        1, 16, 120, 560, 1820, 4368, 8008, 11440, 12870, 11440, 8008, 4368, 1820, 560, 120, 16, 1,
    ],
    &[
        1, 17, 136, 680, 2380, 6188, 12376, 19448, 24310, 24310, 19448, 12376, 6188, 2380, 680,
        136, 17, 1,
    ],
    &[
        1, 18, 153, 816, 3060, 8568, 18564, 31824, 43758, 48620, 43758, 31824, 18564, 8568, 3060,
        816, 153, 18, 1,
    ],
    &[
        1, 19, 171, 969, 3876, 11628, 27132, 50388, 75582, 92378, 92378, 75582, 50388, 27132,
        11628, 3876, 969, 171, 19, 1,
    ],
    &[
        1, 20, 190, 1140, 4845, 15504, 38760, 77520, 125970, 167960, 184756, 167960, 125970, 77520,
        38760, 15504, 4845, 1140, 190, 20, 1,
    ],
    &[
        1, 21, 210, 1330, 5985, 20349, 54264, 116280, 203490, 293930, 352716, 352716, 293930,
        203490, 116280, 54264, 20349, 5985, 1330, 210, 21, 1,
    ],
];

fn powm(i: usize) -> i64 {
    if i % 2 == 0 {
        1
    } else {
        -1
    }
}

const LEN: usize = 21;

pub fn part1(input: &str) -> Output {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            assert!(nums.len() == LEN);

            let mut nums2 = nums.clone();
            let i = (0..LEN)
                .find(|&i| {
                    nums2 = (0..LEN - i - 1).map(|i| nums2[i + 1] - nums2[i]).collect();
                    nums2.iter().all(|&x| x == 0)
                })
                .unwrap()
                + 1;

            PASCAL[i]
                .iter()
                .zip(&nums[LEN - i..])
                .fold((0, -powm(i)), |(res, mul), (coeff, num)| {
                    (res + mul * coeff * num, -mul)
                })
                .0
        })
        .sum::<i64>() as _
}

pub fn part2(input: &str) -> Output {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .rev()
                .collect::<Vec<_>>();
            assert!(nums.len() == LEN);

            let mut nums2 = nums.clone();
            let i = (0..LEN)
                .find(|&i| {
                    nums2 = (0..LEN - i - 1).map(|i| nums2[i + 1] - nums2[i]).collect();
                    nums2.iter().all(|&x| x == 0)
                })
                .unwrap()
                + 1;

            PASCAL[i]
                .iter()
                .zip(&nums[LEN - i..])
                .fold((0, -powm(i)), |(res, mul), (coeff, num)| {
                    (res + mul * coeff * num, -mul)
                })
                .0
        })
        .sum::<i64>() as _
}