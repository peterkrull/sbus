use criterion::{criterion_group, criterion_main};

mod parse_bench;
mod parse_bitvec_bench;

criterion_group!(benches, 
    parse_bench::bench_parser,
    parse_bitvec_bench::bench_parser_bitvec
);
criterion_main!(benches);
