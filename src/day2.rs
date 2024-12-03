use aoc_runner_derive::aoc;

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
    type Item = (u32, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let (i, s) = (self.i, self.s);

        if i >= s.len() {
            return None;
        }

        let a = (s[i] - b'0') as u32;
        let next = s[i + 1];

        if next == b' ' {
            self.i += 2;
            return Some((a, false));
        }

        if next == b'\n' {
            self.i += 2;
            return Some((a, true));
        }

        let b = a * 10 + (next - b'0') as u32;
        self.i += 3;
        Some((b, i + 2 >= s.len() || s[i + 2] == b'\n'))
    }
}

#[inline]
fn is_safe_diff(a: u32, b: u32) -> bool {
    let diff = a.abs_diff(b);
    diff > 0 && diff < 4
}

#[inline]
fn skip_to_eol(iter: &mut Iter) {
    while let Some((_, false)) = iter.next() {}
}

fn check_line_part1(iter: &mut Iter) -> Option<bool> {
    let (first, _) = iter.next()?;
    let (mut prev, _) = iter.next().unwrap();

    if !is_safe_diff(first, prev) {
        skip_to_eol(iter);
        return Some(false);
    }

    let incr = prev > first;

    while let Some((cur, eol)) = iter.next() {
        if incr && prev >= cur {
            if !eol {
                skip_to_eol(iter);
            }
            return Some(false);
        }

        if !incr && cur >= prev {
            if !eol {
                skip_to_eol(iter);
            }
            return Some(false);
        }

        if !is_safe_diff(prev, cur) {
            if !eol {
                skip_to_eol(iter);
            }
            return Some(false);
        }

        if eol {
            return Some(true);
        }

        prev = cur;
    }

    Some(true)
}

fn check_line_part2(mut iter: impl Iterator<Item = u32>) -> bool {
    let first = iter.next().unwrap();
    let mut prev = iter.next().unwrap();

    if !is_safe_diff(first, prev) {
        return false;
    }

    let incr = prev > first;

    while let Some(cur) = iter.next() {
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

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let mut iter = Iter::new(input);
    let mut cnt = 0;

    while let Some(safe) = check_line_part1(&mut iter) {
        if !safe {
            cnt += 1;
        } else {
            // the rest should be safe
            break;
        }
    }

    1000 - cnt
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut iter = Iter::new(input);
    let mut cnt = 0;
    let mut buf: Vec<u32> = Vec::with_capacity(8);

    while let Some((v, eol)) = iter.next() {
        buf.push(v);

        if eol {
            for i in 0..buf.len() {
                let line = buf
                    .iter()
                    .enumerate()
                    .filter(|&(idx, _)| i != idx)
                    .map(|(_, &v)| v);

                if check_line_part2(line) {
                    cnt += 1;
                    break;
                }
            }
            buf.clear();
        }
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EX: &str = indoc! {"
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        7 6 4 2 1
        1 3 6 7 9
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&EX), 996);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&EX), 4);
    }
}
