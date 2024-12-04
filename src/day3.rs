use aoc_runner_derive::aoc;
use memchr::memmem;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    calculate(input.as_bytes())
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let mut s = input.as_bytes();
    let mut sum = 0;

    loop {
        let next_dont = memmem::find(s, b"don't()");

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

        let next_do = memmem::find(from, b"do()");

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
    let mut sum = 0;

    while let Some(pos) = memmem::find(s, b"mul(") {
        let mut a = 0u64;
        let mut b = 0u64;
        let mut next_i = 0usize;
        let mut comma = false;
        let mut closed = false;

        for i in (pos + 4)..=(pos + 11) {
            match s[i] {
                x @ b'0'..=b'9' => {
                    a = a * 10 + (x - b'0') as u64;
                }
                b',' => {
                    if comma {
                        next_i = i + 1;
                        break;
                    }
                    comma = true;
                    b = a;
                    a = 0;
                }
                b')' => {
                    next_i = i + 1;
                    closed = true;
                    break;
                }
                _ => {
                    next_i = i;
                    break;
                }
            }
        }

        if !closed || !comma {
            s = &s[next_i..];
            continue;
        }

        sum += a * b;
        s = &s[next_i..];
    }

    sum
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
