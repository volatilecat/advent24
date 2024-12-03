use aoc_runner_derive::aoc;
use lazy_regex::regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let re = regex!(r"mul\((\d{1,3}),(\d{1,3})\)");
    let mut sum = 0;

    for f in re.captures_iter(input) {
        let a: u32 = f.get(1).unwrap().as_str().parse().unwrap();
        let b: u32 = f.get(2).unwrap().as_str().parse().unwrap();
        sum += a * b;
    }

    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let re = regex!(r"(do\(\))|(don't\(\))|mul\((\d{1,3}),(\d{1,3})\)");
    let mut sum = 0;
    let mut is_on = true;

    for f in re.captures_iter(input) {
        if f.get(1).is_some() {
            is_on = true;
            continue;
        }
        if f.get(2).is_some() {
            is_on = false;
            continue;
        }
        if !is_on {
            continue;
        }
        let a: u32 = f.get(3).unwrap().as_str().parse().unwrap();
        let b: u32 = f.get(4).unwrap().as_str().parse().unwrap();
        sum += a * b;
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
