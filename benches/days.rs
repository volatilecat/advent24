use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paste::paste;

macro_rules! day_input {
    ($day_num:literal) => {
        include_str!(concat!("../input/2024/day", $day_num, ".txt"))
    };
}

macro_rules! benches_day {
    ($day_num:literal) => {
        paste! {
            use advent24::[<day $day_num>];

            pub fn [<bench_day $day_num>](c: &mut Criterion) {
                let mut group = c.benchmark_group(concat!("day", $day_num));
                let input = day_input!($day_num);
                group.bench_function(concat!("day", $day_num, "_part1"), |b| b.iter(|| [<day $day_num>]::part1(black_box(input))));
                group.bench_function(concat!("day", $day_num, "_part2"), |b| b.iter(|| [<day $day_num>]::part2(black_box(input))));
            }
        }
    };
}

macro_rules! benches {
    ($($day_num:literal),*) => {
        paste! {
            $(
                benches_day!($day_num);
            )*

            criterion_group!(benches, $([<bench_day $day_num>]),*);
            criterion_main!(benches);
        }
    };
}

benches!(1, 2, 3, 4);
