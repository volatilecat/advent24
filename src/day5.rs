use std::simd::prelude::*;

pub fn part1(input: &str) -> u32 {
    unsafe { calculate(input, get_correct) }
}

pub fn part2(input: &str) -> u32 {
    unsafe { calculate(input, get_incorrect) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn calculate<F>(input: &str, f: F) -> u32
where
    F: Fn(&[u8], &[u128; 100]) -> Option<u8>,
{
    let mut b = input.as_bytes();
    let mut table = [0u128; 100];

    loop {
        let req = (b[0] - b'0') * 10 + (b[1] - b'0');
        let page = (b[3] - b'0') * 10 + (b[4] - b'0');

        table[page as usize] |= 1 << req as u128;

        b = &b[6..];

        if b[0] == b'\n' {
            b = &b[1..];
            break;
        }
    }

    let mut sum = 0;

    let mut updates = [0u8; 64];

    while let Some(eol) = find_newline(b) {
        let count = parse_updates(&b[..eol], &mut updates);

        if let Some(page) = f(&updates[..count], &table) {
            sum += page as u32;
        }

        b = &b[(eol + 1)..];
    }

    sum
}

fn find_newline(mut b: &[u8]) -> Option<usize> {
    let check = u8x32::splat(b'\n');
    let mut pos: usize = 0;
    loop {
        if b.len() < 32 {
            for i in 0..b.len() {
                if b[i] == b'\n' {
                    return Some(pos + i);
                }
            }
            return None;
        }
        let a = u8x32::from_slice(&b[..32]);
        if let Some(index) = a.simd_eq(check).first_set() {
            return Some(pos + index);
        }
        b = &b[32..];
        pos += 32;
    }
}

fn parse_updates(mut b: &[u8], updates: &mut [u8; 64]) -> usize {
    let ascii0_16 = u8x16::splat(b'0');
    let ten_16 = u8x16::splat(10);

    let ascii0_32 = u8x32::splat(b'0');
    let ten_32 = u8x32::splat(10);

    let mut count = 0;

    while !b.is_empty() {
        if b.len() >= 65 {
            let x = u8x32::from_slice(&b[..32]);
            let y = u8x32::from_slice(&b[33..]);

            let hi: u8x32 = simd_swizzle!(
                x,
                y,
                [
                    0, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, // 11
                    32, 35, 38, 41, 44, 47, 50, 53, 56, 59, 62, // 11
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                ]
            );
            let lo: u8x32 = simd_swizzle!(
                x,
                y,
                [
                    1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, // 11
                    33, 36, 39, 42, 45, 48, 51, 54, 57, 60, 63, // 11
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                ]
            );

            let r = (hi - ascii0_32) * ten_32 + (lo - ascii0_32);
            r.copy_to_slice(&mut updates[count..]);

            b = &b[66..];
            count += 22;

            continue;
        }

        if b.len() < 33 {
            loop {
                let x = (b[0] - b'0') * 10 + (b[1] - b'0');

                updates[count] = x;
                count += 1;

                if b.len() < 3 {
                    return count;
                }

                b = &b[3..];
            }
        }

        let x = u8x32::from_slice(&b[..32]);

        let hi: u8x16 = simd_swizzle!(
            x,
            [
                0, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, // 11
                0, 0, 0, 0, 0, //
            ]
        );

        let lo: u8x16 = simd_swizzle!(
            x,
            [
                1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, // 11
                0, 0, 0, 0, 0, //
            ]
        );

        let r = (hi - ascii0_16) * ten_16 + (lo - ascii0_16);
        r.copy_to_slice(&mut updates[count..]);
        b = &b[33..];
        count += 11;
    }

    count
}

#[inline]
fn get_correct(updates: &[u8], table: &[u128; 100]) -> Option<u8> {
    let mut printed = 0u128;
    let mut forbidden = 0u128;

    for page in updates {
        let page = *page as usize;
        if forbidden & (1 << page) != 0 {
            return None;
        }
        forbidden |= table[page] & !printed;
        printed |= 1 << page;
    }

    Some(updates[updates.len() / 2])
}

#[inline]
fn get_incorrect(updates: &[u8], table: &[u128; 100]) -> Option<u8> {
    let mut printed = 0u128;
    let mut forbidden = 0u128;
    let mut incorrect = false;

    for page in updates {
        let page = *page as usize;

        if forbidden & (1 << page) != 0 {
            incorrect = true;
        }

        forbidden |= table[page] & !printed;
        printed |= 1 << page;
    }

    if !incorrect {
        return None;
    }

    for page in updates {
        let page = *page as usize;

        let req_cnt = (table[page] & printed).count_ones() as usize;

        if req_cnt == updates.len() / 2 {
            return Some(page as u8);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EX: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EX), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EX), 123);
    }
}
