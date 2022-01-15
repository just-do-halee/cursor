use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cursor::{Cursor, CursorTrait};

fn bench(c: &mut Criterion) {
    let dummy = &black_box([1u8; 100]);
    c.bench_function("just iterator", |b| {
        b.iter(|| for _ in dummy {});
    });
    c.bench_function("normal cursor", |b| {
        b.iter(|| {
            let mut cursor = Cursor::new(dummy);
            cursor.next_to_last();
        });
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
