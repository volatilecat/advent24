use aoc_runner_derive::aoc;
use memchr::memmem;

const XMAS: u32 = u32::from_be_bytes([b'X', b'M', b'A', b'S']);
const SAMX: u32 = u32::from_be_bytes([b'S', b'A', b'M', b'X']);

const MSMS: u32 = u32::from_be_bytes([b'M', b'S', b'M', b'S']);
const MSSM: u32 = u32::from_be_bytes([b'M', b'S', b'S', b'M']);
const SMMS: u32 = u32::from_be_bytes([b'S', b'M', b'M', b'S']);
const SMSM: u32 = u32::from_be_bytes([b'S', b'M', b'S', b'M']);

#[inline(always)]
fn word(b: &[u8], i1: usize, i2: usize, i3: usize, i4: usize) -> u32 {
    unsafe {
        (*b.get_unchecked(i1) as u32) << 24
            | (*b.get_unchecked(i2) as u32) << 16
            | (*b.get_unchecked(i3) as u32) << 8
            | *b.get_unchecked(i4) as u32
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe { part1_impl::<141, 140>(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part1_impl<const W: usize, const H: usize>(input: &str) -> u32 {
    let b = input.as_bytes();
    let mut cnt = 0;

    cnt += memmem::find_iter(b, b"XMAS").count() as u32;
    cnt += memmem::find_iter(b, b"SAMX").count() as u32;

    for row in 0..(H - 3) {
        for col in 0..(W - 1) {
            let p = W * row + col;

            let w = word(b, p, p + W, p + W * 2, p + W * 3);
            cnt += (w == XMAS) as u32 + (w == SAMX) as u32;

            if col < (W - 4) {
                let w = word(b, p, p + W + 1, p + W * 2 + 2, p + W * 3 + 3);
                cnt += (w == XMAS) as u32 + (w == SAMX) as u32;
            }

            if col >= 3 {
                let w = word(b, p, p + (W - 1), p + (W - 1) * 2, p + (W - 1) * 3);
                cnt += (w == XMAS) as u32 + (w == SAMX) as u32;
            }
        }
    }

    cnt
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    unsafe { part2_impl::<141, 140>(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_impl<const W: usize, const H: usize>(input: &str) -> u32 {
    let b = input.as_bytes();
    let mut cnt = 0;

    for row in 1..(H - 1) {
        for col in 1..(W - 2) {
            let p = W * row + col;
            if *b.get_unchecked(p) == b'A' {
                let w = word(b, p - W - 1, p + W + 1, p - W + 1, p + W - 1);
                cnt += (w == MSMS) as u32
                    + (w == MSSM) as u32
                    + (w == SMMS) as u32
                    + (w == SMSM) as u32;
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
        assert_eq!(unsafe { part1_impl::<11, 10>(EX) }, 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(unsafe { part2_impl::<11, 10>(EX) }, 9);
    }
}
