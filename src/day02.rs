use crate::Output;

#[derive(Debug, Copy, Clone)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    fn new() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    fn leq(self, other: Self) -> bool {
        self.r <= other.r && self.g <= other.g && self.b <= other.b
    }

    fn max(self, other: Self) -> Self {
        Self {
            r: self.r.max(other.r),
            g: self.g.max(other.g),
            b: self.b.max(other.b),
        }
    }

    fn power(self) -> u32 {
        self.r as u32 * self.g as u32 * self.b as u32
    }
}

fn parser(input: &str) -> nom::IResult<&str, Vec<(u8, Vec<Rgb>)>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, line_ending, u8};
    use nom::combinator::map;
    use nom::multi::{many0, separated_list0};
    use nom::sequence::{preceded, separated_pair, terminated};

    many0(terminated(
        separated_pair(
            preceded(tag("Game "), u8),
            tag(": "),
            separated_list0(
                tag("; "),
                map(
                    separated_list0(tag(", "), separated_pair(u8, char(' '), alpha1)),
                    |lst| {
                        lst.into_iter()
                            .fold(Rgb::new(), |rgb, (count, color)| match color {
                                "red" => Rgb {
                                    r: rgb.r + count,
                                    ..rgb
                                },
                                "green" => Rgb {
                                    g: rgb.g + count,
                                    ..rgb
                                },
                                "blue" => Rgb {
                                    b: rgb.b + count,
                                    ..rgb
                                },
                                _ => rgb,
                            })
                    },
                ),
            ),
        ),
        line_ending,
    ))(input)
}

fn parse_input(input: &str) -> Vec<(u8, Vec<Rgb>)> {
    parser(input).unwrap().1
}

pub fn part1(input: &str) -> Output {
    let data = parse_input(input);

    let max_values = Rgb {
        r: 12,
        g: 13,
        b: 14,
    };

    let res = data.iter().fold(0, |acc, (id, game)| {
        if game.iter().all(|rgb| rgb.leq(max_values)) {
            acc + *id as u32
        } else {
            acc
        }
    });

    res.into()
}

pub fn part2(input: &str) -> Output {
    let data = parse_input(input);

    let res: u32 = data
        .iter()
        .flat_map(|(_, game)| game.iter().copied().reduce(|a, b| a.max(b)).map(Rgb::power))
        .sum();

    res.into()
}
