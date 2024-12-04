use aoc_runner_derive::aoc;
use memchr::memmem;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u64 {
    part1_impl::<141, 140>(input)
}

#[inline(always)]
fn part1_impl<const W: usize, const H: usize>(input: &str) -> u64 {
    let b = input.as_bytes();
    let mut cnt = 0;

    cnt += memmem::find_iter(b, b"XMAS").count() as u64;
    cnt += memmem::find_iter(b, b"SAMX").count() as u64;

    // vert

    for row in 0..(H - 3) {
        for col in 0..(W - 1) {
            if b[W * row + col] == b'X'
                && b[W * row + col + W] == b'M'
                && b[W * row + col + W * 2] == b'A'
                && b[W * row + col + W * 3] == b'S'
            {
                cnt += 1;
            } else if b[W * row + col] == b'S'
                && b[W * row + col + W] == b'A'
                && b[W * row + col + W * 2] == b'M'
                && b[W * row + col + W * 3] == b'X'
            {
                cnt += 1;
            }
        }
    }

    // diag \

    for row in 0..(H - 3) {
        for col in 0..(W - 4) {
            if b[W * row + col] == b'X'
                && b[W * row + col + W + 1] == b'M'
                && b[W * row + col + W * 2 + 2] == b'A'
                && b[W * row + col + W * 3 + 3] == b'S'
            {
                cnt += 1;
            } else if b[W * row + col] == b'S'
                && b[W * row + col + W + 1] == b'A'
                && b[W * row + col + W * 2 + 2] == b'M'
                && b[W * row + col + W * 3 + 3] == b'X'
            {
                cnt += 1;
            }
        }
    }

    // diag /

    for row in 0..(H - 3) {
        for col in 3..(W - 1) {
            if b[W * row + col] == b'X'
                && b[W * row + col + (W - 1)] == b'M'
                && b[W * row + col + (W - 1) * 2] == b'A'
                && b[W * row + col + (W - 1) * 3] == b'S'
            {
                cnt += 1;
            } else if b[W * row + col] == b'S'
                && b[W * row + col + (W - 1)] == b'A'
                && b[W * row + col + (W - 1) * 2] == b'M'
                && b[W * row + col + (W - 1) * 3] == b'X'
            {
                cnt += 1;
            }
        }
    }

    cnt
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u64 {
    part2_impl::<141, 140>(input)
}

#[inline(always)]
fn part2_impl<const W: usize, const H: usize>(input: &str) -> u64 {
    let mut cnt = 0;

    let b = input.as_bytes();

    for row in 1..(H - 1) {
        for col in 1..(W - 2) {
            if b[W * row + col] != b'A' {
                continue;
            }

            if b[W * row + col - W - 1] == b'M' && b[W * row + col + W + 1] == b'S'
                || b[W * row + col - W - 1] == b'S' && b[W * row + col + W + 1] == b'M'
            {
                if b[W * row + col - W + 1] == b'M' && b[W * row + col + W - 1] == b'S'
                    || b[W * row + col - W + 1] == b'S' && b[W * row + col + W - 1] == b'M'
                {
                    cnt += 1;
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
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1_impl::<11, 10>(EX), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_impl::<11, 10>(EX), 9);
    }
}
