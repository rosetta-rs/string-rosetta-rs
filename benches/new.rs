#![allow(
    clippy::clone_on_copy,
    clippy::useless_conversion,
    clippy::clone_double_ref
)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

mod fixture;

type StringCow<'s> = std::borrow::Cow<'s, str>;

fn bench_new(c: &mut Criterion) {
    let mut group = c.benchmark_group("new");
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
        group.bench_with_input(BenchmarkId::new("StringCow::Owned", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| StringCow::Owned(String::from(fixture)))
        });

        group.bench_with_input(BenchmarkId::new("ArcStr::from", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| arcstr::ArcStr::from(fixture))
        });
        group.bench_with_input(BenchmarkId::new("CompactString::new", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| compact_str::CompactString::new(fixture))
        });
        group.bench_with_input(BenchmarkId::new("EcoString::from", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| ecow::EcoString::from(fixture))
        });
        group.bench_with_input(
            BenchmarkId::new("flexstr::SharedStr::from_ref", len),
            &len,
            |b, _| {
                let fixture = criterion::black_box(*fixture);
                b.iter(|| flexstr::SharedStr::from_ref(fixture))
            },
        );
        group.bench_with_input(BenchmarkId::new("HipStr::from", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| hipstr::HipStr::from(fixture))
        });
        group.bench_with_input(BenchmarkId::new("ImString::from", len), &len, |b, _| {
            let fixture = criterion::black_box(*fixture);
            b.iter(|| imstr::ImString::from(fixture))
        });
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
    }
    group.finish();
}

fn bench_new_static(c: &mut Criterion) {
    let mut group = c.benchmark_group("new_static");
    for fixture in fixture::SAMPLES {
        let len = fixture.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(
            BenchmarkId::new("StringCow::Borrowed", len),
            &len,
            |b, _| {
                let fixture = criterion::black_box(*fixture);
                b.iter(|| StringCow::Borrowed(fixture))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("flexstr::SharedStr::from_static", len),
            &len,
            |b, _| {
                let fixture = criterion::black_box(*fixture);
                b.iter(|| flexstr::SharedStr::from_static(fixture))
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
    }
    group.finish();
}

criterion_group!(benches, bench_new, bench_new_static);
criterion_main!(benches);
