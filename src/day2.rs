use aoc_runner_derive::aoc;

fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32> + '_> + '_ {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split(" ").map(|v| v.parse().unwrap()))
}

#[inline]
fn is_safe_diff(a: u32, b: u32) -> bool {
    let diff = a.abs_diff(b);
    diff > 0 && diff < 4
}

fn is_safe(mut line: impl Iterator<Item = u32>) -> bool {
    let first = line.next().unwrap();
    let mut prev = line.next().unwrap();

    if !is_safe_diff(first, prev) {
        return false;
    }

    let incr = prev > first;

    for cur in line {
        if incr && prev >= cur {
            return false;
        }

        if !incr && cur >= prev {
            return false;
        }

        if !is_safe_diff(prev, cur) {
            return false;
        }

        prev = cur;
    }

    true
}

fn is_safe_skip(line: impl Iterator<Item = u32>, idx: usize) -> bool {
    let line = line.enumerate().filter(|&(i, _)| i != idx).map(|(_, v)| v);
    is_safe(line)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let mut cnt = 0;
    for line in parse(input) {
        if is_safe(line) {
            cnt += 1;
        }
    }
    cnt
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut cnt = 0;
    let mut buf: Vec<u32> = Vec::with_capacity(8);

    for line in parse(input) {
        buf.extend(line);

        if is_safe(buf.iter().copied()) {
            cnt += 1;
        } else {
            for i in 0..buf.len() {
                if is_safe_skip(buf.iter().copied(), i) {
                    cnt += 1;
                    break;
                }
            }
        }

        buf.clear();
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
