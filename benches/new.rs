#![allow(
    clippy::clone_on_copy,
    clippy::useless_conversion,
    clippy::clone_double_ref
)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

mod fixture;

type StringCow<'s> = std::borrow::Cow<'s, str>;

fn bench_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone");
    for fixture in fixture::SAMPLES {
        let len = fixture.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| String::from(fixture))
        });
        group.bench_with_input(BenchmarkId::new("Box<str>", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| Box::<str>::from(fixture))
        });
        group.bench_with_input(BenchmarkId::new("Arc<str>", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| std::sync::Arc::<str>::from(fixture))
        });
        group.bench_with_input(
            BenchmarkId::new("StringCow::Borrowed", len),
            &len,
            |b, _| {
                let fixture = criterion::black_box(*fixture);
                b.iter(|| StringCow::Borrowed(fixture))
            },
        );
        group.bench_with_input(BenchmarkId::new("StringCow::Owned", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| StringCow::Owned(String::from(fixture)))
        });
        group.bench_with_input(BenchmarkId::new("CompactStr::new", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| compact_str::CompactStr::new(fixture))
        });
        group.bench_with_input(
            BenchmarkId::new("flexstr::SharedStr::from_static", len),
            &len,
            |b, _| {
                let fixture = criterion::black_box(*fixture);
                b.iter(|| flexstr::SharedStr::from_static(fixture))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("flexstr::SharedStr::from_ref", len),
            &len,
            |b, _| {
                let fixture = criterion::black_box(*fixture);
                b.iter(|| flexstr::SharedStr::from_ref(fixture))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("flexstr::SharedStr::from_heap", len),
            &len,
            |b, _| {
                let fixture = criterion::black_box(*fixture);
                b.iter(|| flexstr::SharedStr::from_heap(std::sync::Arc::from(fixture)))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("KString::from_static", len),
            &len,
            |b, _| {
                let fixture = criterion::black_box(*fixture);
                b.iter(|| kstring::KString::from_static(fixture))
            },
        );
        group.bench_with_input(BenchmarkId::new("KString::from_ref", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| kstring::KString::from_ref(fixture))
        });
        group.bench_with_input(
            BenchmarkId::new("KString::from_string", len),
            &len,
            |b, _| {
                let fixture = criterion::black_box(*fixture);
                b.iter(|| kstring::KString::from_string(String::from(fixture)))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("smartstring::String::new", len),
            &len,
            |b, _| {
                let fixture = criterion::black_box(*fixture);
                b.iter(|| smartstring::alias::String::from(fixture))
            },
        );
        group.bench_with_input(BenchmarkId::new("SmolStr::new", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| smol_str::SmolStr::new(fixture))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_clone);
criterion_main!(benches);
