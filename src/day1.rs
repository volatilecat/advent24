use aoc_runner_derive::aoc;

use crate::util::Counter;

const INPUT_CAP: usize = 1000;

fn parse(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let mut a = Vec::with_capacity(INPUT_CAP);
    let mut b = Vec::with_capacity(INPUT_CAP);

    parse(input).for_each(|(x, y)| {
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
fn part2(input: &str) -> u64 {
    let mut list = Vec::with_capacity(INPUT_CAP);
    let mut count = Counter::with_capacity(INPUT_CAP);

    parse(input).for_each(|(x, y)| {
        list.push(x);
        count.add(y);
    });

    let mut score = 0;

    for i in list {
        score += i * count.get(&i);
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EX: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
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
