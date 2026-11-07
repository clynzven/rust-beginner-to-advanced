use std::thread;

fn main() {
    let numbers: Vec<u64> = (1..=1_000_000).collect();
    let chunks: Vec<&[u64]> = numbers.chunks(250_000).collect();

    let partial_sums: Vec<u64> = thread::scope(|s| {
        let handles: Vec<_> = chunks
            .iter()
            .map(|chunk| s.spawn(move || chunk.iter().sum::<u64>()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    println!("partial sums: {partial_sums:?}");
    println!("total: {}", partial_sums.iter().sum::<u64>());
}
