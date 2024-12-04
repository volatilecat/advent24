#[cfg(not(feature = "ci"))]
aoc_runner_derive::aoc_main! { lib = advent24 }

#[cfg(feature = "ci")]
fn main() {}
