#[cfg(not(feature = "run"))]
fn main() {}

#[cfg(feature = "run")]
fn main() {
    macro_rules! check_answer {
        ($day:literal, $part:literal, $r:expr, $a:literal) => {
            if $r == $a {
                println!(concat!("Day ", $day, " - Part ", $part, " : {} ✓"), $r);
            } else {
                println!(
                    concat!("Day ", $day, " - Part ", $part, " : {} != {}"),
                    $r, $a,
                );
            }
        };
    }

    macro_rules! run_days {
        ($($day:literal : ($a1:literal, $a2:literal)),* $(,)?) => {$(
            paste::paste! {
                const [<INPUT_ $day>]: &str = include_str!(concat!("../input/2024/day", $day, ".txt"));

                let r1 = advent24::[<day $day>]::part1([<INPUT_ $day>]);
                check_answer!($day, 1, r1, $a1);

                let r2 = advent24::[<day $day>]::part2([<INPUT_ $day>]);
                check_answer!($day, 2, r2, $a2);

                println!();
            }
        )*};
    }

    run_days! {
        1 : (1189304, 24349736),
        2 : (230, 301),
        3 : (188192787, 113965544),
        4 : (2603, 1965),
        5 : (4957, 6938),
    }
}
