pub fn parse_num(x: &str) -> u64 {
    x.bytes().fold(0, |acc, x| acc * 10 + (x & 0xf) as u64)
}
