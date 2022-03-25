#![allow(
    clippy::clone_on_copy,
    clippy::useless_conversion,
    clippy::clone_double_ref
)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

mod fixture;

type StringCow<'s> = std::borrow::Cow<'s, str>;

fn bench_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("access");
    for fixture in fixture::SAMPLES {
        let len = fixture.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("&'static str", len), &len, |b, _| {
            let uut = *fixture;
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, _| {
            let uut = String::from(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
        group.bench_with_input(BenchmarkId::new("Box<str>", len), &len, |b, _| {
            let uut = Box::<str>::from(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
        group.bench_with_input(BenchmarkId::new("Arc<str>", len), &len, |b, _| {
            let uut = std::sync::Arc::<str>::from(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
        group.bench_with_input(
            BenchmarkId::new("StringCow::Borrowed", len),
            &len,
            |b, _| {
                let uut = StringCow::Borrowed(*fixture);
                let uut = criterion::black_box(uut);
                b.iter(|| uut.is_empty())
            },
        );
        group.bench_with_input(BenchmarkId::new("StringCow::Owned", len), &len, |b, _| {
            let uut = StringCow::Owned(String::from(*fixture));
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
        group.bench_with_input(BenchmarkId::new("CompactStr::new", len), &len, |b, _| {
            let uut = compact_str::CompactStr::new(fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
        group.bench_with_input(
            BenchmarkId::new("flexstr::SharedStr::from_static", len),
            &len,
            |b, _| {
                let uut = flexstr::SharedStr::from_static(*fixture);
                let uut = criterion::black_box(uut);
                b.iter(|| uut.is_empty())
            },
        );
        group.bench_with_input(
            BenchmarkId::new("flexstr::SharedStr::from_ref", len),
            &len,
            |b, _| {
                let uut = flexstr::SharedStr::from_ref(*fixture);
                let uut = criterion::black_box(uut);
                b.iter(|| uut.is_empty())
            },
        );
        group.bench_with_input(
            BenchmarkId::new("flexstr::SharedStr::from_heap", len),
            &len,
            |b, _| {
                let uut = flexstr::SharedStr::from_heap(std::sync::Arc::from(*fixture));
                let uut = criterion::black_box(uut);
                b.iter(|| uut.is_empty())
            },
        );
        group.bench_with_input(
            BenchmarkId::new("KString::from_static", len),
            &len,
            |b, _| {
                let uut = kstring::KString::from_static(*fixture);
                let uut = criterion::black_box(uut);
                b.iter(|| uut.is_empty())
            },
        );
        group.bench_with_input(BenchmarkId::new("KString::from_ref", len), &len, |b, _| {
            let uut = kstring::KString::from_ref(*fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
        group.bench_with_input(
            BenchmarkId::new("KString::from_string", len),
            &len,
            |b, _| {
                let uut = kstring::KString::from_string(String::from(*fixture));
                let uut = criterion::black_box(uut);
                b.iter(|| uut.is_empty())
            },
        );
        group.bench_with_input(
            BenchmarkId::new("smartstring::String::new", len),
            &len,
            |b, _| {
                let uut = smartstring::alias::String::from(*fixture);
                let uut = criterion::black_box(uut);
                b.iter(|| uut.is_empty())
            },
        );
        group.bench_with_input(BenchmarkId::new("SmolStr::new", len), &len, |b, _| {
            let uut = smol_str::SmolStr::new(fixture);
            let uut = criterion::black_box(uut);
            b.iter(|| uut.is_empty())
        });
    }
    group.finish();
}

criterion_group!(benches, bench_access);
criterion_main!(benches);
