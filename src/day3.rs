use aoc_runner_derive::aoc;
use memchr::{arch::all::packedpair::HeuristicFrequencyRank, memmem};

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    calculate(input.as_bytes())
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let dont_finder = build_finder(b"don't()");
    let do_finder = build_finder(b"do()");

    let mut s = input.as_bytes();
    let mut sum = 0;

    loop {
        let next_dont = dont_finder.find(s);

        let limit = if let Some(next_dont) = next_dont {
            next_dont
        } else {
            s.len()
        };

        sum += calculate(&s[0..limit]);

        if limit == s.len() {
            break;
        }

        let from = &s[(limit + 7)..];

        let next_do = do_finder.find(from);

        if let Some(next_do) = next_do {
            s = &from[(next_do + 4)..];
        } else {
            break;
        }
    }

    sum
}

#[inline(always)]
fn calculate(mut s: &[u8]) -> u64 {
    let finder = build_finder(b"mul(");

    let mut sum = 0;

    while let Some(pos) = finder.find(s) {
        let mut a = 0u64;
        let mut b = 0u64;
        let mut comma = false;
        let mut closed = false;

        for i in (pos + 4)..=(pos + 11) {
            match s[i] {
                x @ b'0'..=b'9' => {
                    a = a * 10 + (x - b'0') as u64;
                }
                b',' => {
                    if comma {
                        s = &s[(i + 1)..];
                        break;
                    }
                    comma = true;
                    b = a;
                    a = 0;
                }
                b')' => {
                    s = &s[(i + 1)..];
                    closed = true;
                    break;
                }
                _ => {
                    s = &s[i..];
                    break;
                }
            }
        }

        if !closed || !comma {
            continue;
        }

        sum += a * b;
    }

    sum
}

#[inline(always)]
fn build_finder(needle: &[u8]) -> memmem::Finder {
    memmem::FinderBuilder::new().build_forward_with_ranker(Ranker, needle)
}

struct Ranker;

impl HeuristicFrequencyRank for Ranker {
    fn rank(&self, byte: u8) -> u8 {
        const TABLE: [u8; 256] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 15, 16, 0, 16, 15, 16, 15, 20, 245, 240, 16, 16, 120, 15, 0, 15, 38, 65, 64,
            66, 64, 62, 61, 62, 62, 60, 15, 16, 15, 0, 14, 13, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 16, 0, 0, 15, 0, 15, 8, 79, 16,
            0, 95, 0, 0, 0, 113, 114, 20, 55, 0, 0, 32, 15, 35, 97, 0, 95, 0, 16, 0, 15, 0, 15, 16,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        TABLE[byte as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let s = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(s), 161);
    }

    #[test]
    fn part2_example() {
        let s = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(s), 48);
    }
}
