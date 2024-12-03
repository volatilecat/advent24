use std::collections::HashMap;

use aoc_runner_derive::aoc;
use rustc_hash::FxBuildHasher;

const INPUT_CAP: usize = 1000;

struct Iter<'a> {
    s: &'a [u8],
    i: usize,
}

impl<'a> Iter<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            s: s.as_bytes(),
            i: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let (i, s) = (self.i, self.s);

        if i >= s.len() {
            return None;
        }

        let a = (s[i] - b'0') as u64 * 10_000
            + (s[i + 1] - b'0') as u64 * 1_000
            + (s[i + 2] - b'0') as u64 * 100
            + (s[i + 3] - b'0') as u64 * 10
            + (s[i + 4] - b'0') as u64;

        let b = (s[i + 8] - b'0') as u64 * 10_000
            + (s[i + 9] - b'0') as u64 * 1_000
            + (s[i + 10] - b'0') as u64 * 100
            + (s[i + 11] - b'0') as u64 * 10
            + (s[i + 12] - b'0') as u64;

        self.i += 14;

        Some((a, b))
    }
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let mut a = Vec::with_capacity(INPUT_CAP);
    let mut b = Vec::with_capacity(INPUT_CAP);

    Iter::new(input).for_each(|(x, y)| {
        a.push(x);
        b.push(y);
    });

    a.sort_unstable();
    b.sort_unstable();

    let mut sum = 0;

    for (x, y) in a.iter().zip(b.iter()) {
        sum += x.abs_diff(*y);
    }

    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    let mut list = Vec::with_capacity(INPUT_CAP);
    let mut count: HashMap<u64, u64, _> =
        HashMap::with_capacity_and_hasher(INPUT_CAP, FxBuildHasher);

    Iter::new(input).for_each(|(x, y)| {
        list.push(x);
        count.entry(y).and_modify(|e| *e += 1).or_insert(1);
    });

    let mut score = 0;

    for i in list {
        score += i * count.get(&i).copied().unwrap_or(0)
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EX: &str = indoc! {"
        00003   00004
        00004   00003
        00002   00005
        00001   00003
        00003   00009
        00003   00003
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EX), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EX), 31);
    }
}
