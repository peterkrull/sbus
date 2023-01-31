use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::hex;

fn bench_parser_original(c: &mut Criterion) {

    let mut parser = sbus::SBusPacketParser::new();

    let bytes =
        hex!("00 0F E0 03 1F 58 C0 07 16 B0 80 05 2C 60 01 0B F8 C0 07 00 00 00 00 00 03 00");
    
    c.bench_function("parser_original", |b| b.iter(||{
        parser.push_bytes(&bytes);
        let msg = parser.try_parse_original().unwrap();
        black_box(msg);
    }));
}

fn bench_parser_bitvec(c: &mut Criterion) {

    let mut parser = sbus::SBusPacketParser::new();

    let bytes =
        hex!("00 0F E0 03 1F 58 C0 07 16 B0 80 05 2C 60 01 0B F8 C0 07 00 00 00 00 00 03 00");
    
    c.bench_function("parser_bitvec", |b| b.iter(||{
        
        parser.push_bytes(&bytes);
        let msg = parser.try_parse().unwrap();
        
        black_box(msg);
    }));
}


criterion_group!(benches, 
    bench_parser_original,
    bench_parser_bitvec
);
criterion_main!(benches);
