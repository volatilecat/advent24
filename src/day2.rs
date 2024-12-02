use aoc_runner_derive::aoc;
use itertools::Itertools;

fn parse(input: &str) -> impl Iterator<Item = Vec<u32>> + '_ {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split(" ").map(|v| v.parse().unwrap()).collect_vec())
}

#[inline]
fn is_safe_diff(a: u32, b: u32) -> bool {
    let diff = a.abs_diff(b);
    diff > 0 && diff < 4
}

fn is_safe(line: &Vec<u32>) -> bool {
    let (a, b) = (line[0], line[1]);

    if !is_safe_diff(a, b) {
        return false;
    }

    let incr = b > a;

    for i in 1..line.len() {
        let (a, b) = (line[i - 1], line[i]);

        if incr && a >= b {
            return false;
        }

        if !incr && b >= a {
            return false;
        }

        if !is_safe_diff(a, b) {
            return false;
        }
    }

    true
}

fn is_safe_skip(line: &Vec<u32>, remove: usize) -> bool {
    let line: Vec<u32> = line
        .iter()
        .copied()
        .take(remove)
        .chain(line.iter().copied().skip(remove + 1))
        .collect();

    is_safe(&line)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    parse(input).filter(is_safe).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut cnt = 0;

    for line in parse(input) {
        if is_safe(&line) {
            cnt += 1
        } else {
            for i in 0..line.len() {
                if is_safe_skip(&line, i) {
                    cnt += 1;
                    break;
                }
            }
        }
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EX: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&EX), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&EX), 4);
    }
}
