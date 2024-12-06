pub fn part1(input: &str) -> u32 {
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

    while let Some(eol) = memchr::memchr(b'\n', b) {
        if let Some(page) = get_correct(&b[..eol], &table) {
            sum += page as u32;
        }
        b = &b[(eol + 1)..];
    }

    sum
}

#[inline]
fn get_correct(l: &[u8], table: &[u128; 100]) -> Option<u8> {
    let mut printed = 0u128;
    let mut forbidden = 0u128;

    for i in 0..((l.len() + 1) / 3) {
        let page = (l[i * 3] - b'0') * 10 + (l[i * 3 + 1] - b'0');

        if forbidden & (1 << page) != 0 {
            return None;
        }

        forbidden |= table[page as usize] & !printed;
        printed |= 1 << page;
    }

    let m = l.len() / 2 - 1;
    Some((l[m] - b'0') * 10 + (l[m + 1] - b'0'))
}

pub fn part2(input: &str) -> u32 {
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

    while let Some(eol) = memchr::memchr(b'\n', b) {
        if let Some(page) = get_incorrect(&b[..eol], &table) {
            sum += page as u32;
        }
        b = &b[(eol + 1)..];
    }

    sum
}

#[inline]
fn get_incorrect(l: &[u8], table: &[u128; 100]) -> Option<u8> {
    let pages_len = (l.len() + 1) / 3;
    let mut printed = 0u128;
    let mut forbidden = 0u128;
    let mut incorrect = false;

    for i in 0..pages_len {
        let page = (l[i * 3] - b'0') * 10 + (l[i * 3 + 1] - b'0');

        if forbidden & (1 << page) != 0 {
            incorrect = true;
        }

        forbidden |= table[page as usize] & !printed;
        printed |= 1 << page;
    }

    if !incorrect {
        return None;
    }

    for i in 0..pages_len {
        let page = ((l[i * 3] - b'0') * 10 + (l[i * 3 + 1] - b'0')) as usize;

        let req_cnt = (table[page] & printed).count_ones() as usize;

        if req_cnt == pages_len / 2 {
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
