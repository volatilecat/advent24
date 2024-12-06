use criterion::{criterion_group, criterion_main, Criterion};
use paste::paste;
use std::hint::black_box;

macro_rules! day_input {
    ($day:literal) => {
        include_str!(concat!("../input/2024/day", $day, ".txt"))
    };
}

macro_rules! benches_day {
    ($day:literal) => {
        paste! {
            use advent24::[<day $day>];

            pub fn [<bench_day $day>](c: &mut Criterion) {
                let mut group = c.benchmark_group(concat!("day", $day));
                let input = day_input!($day);
                group.bench_function(concat!("day", $day, "_part1"), |b| b.iter(|| [<day $day>]::part1(black_box(input))));
                group.bench_function(concat!("day", $day, "_part2"), |b| b.iter(|| [<day $day>]::part2(black_box(input))));
            }
        }
    };
}

macro_rules! benches {
    ($($day:literal),*) => {
        paste! {
            $(
                benches_day!($day);
            )*

            criterion_group!(benches, $([<bench_day $day>]),*);
            criterion_main!(benches);
        }
    };
}

benches!(1, 2, 3, 4, 5);
