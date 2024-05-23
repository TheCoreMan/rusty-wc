use std::fs;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusty_wc::frequency;

fn criterion_benchmark(c: &mut Criterion) {
    let file_contents = fs::read_to_string("LICENSE").unwrap();

    c.bench_function("frequency in license", |b| b.iter(|| frequency::count_frequency_of_words_in_content(black_box(&file_contents))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);