use std::hint::black_box;
use std::time::Instant;

fn sum_of_squares_loop(v: &[u64]) -> u64 {
    let mut total = 0;
    for i in 0..v.len() {
        total += v[i] * v[i];
    }
    total
}

fn sum_of_squares_iter(v: &[u64]) -> u64 {
    v.iter().map(|x| x * x).sum()
}

fn time<F: Fn() -> u64>(label: &str, f: F) {
    let start = Instant::now();
    let mut r = 0;
    for _ in 0..20 {
        r = black_box(f());
    }
    let per_run = start.elapsed() / 20;
    let fast = if per_run.as_micros() < 5_000 { "under 5 ms" } else { "over 5 ms" };
    println!("{label:<8} result={r} {fast}");
}

fn main() {
    let data: Vec<u64> = (0..2_000_000).collect();
    time("loop", || sum_of_squares_loop(black_box(&data)));
    time("iter", || sum_of_squares_iter(black_box(&data)));
}
