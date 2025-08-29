#![allow(
    clippy::clone_on_copy,
    clippy::useless_conversion,
    clippy::explicit_auto_deref,
    noop_method_call
)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

mod fixture;

type StringCow<'s> = std::borrow::Cow<'s, str>;

fn bench_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone");
    group.plot_config(
        criterion::PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic),
    );
    for fixture in fixture::SAMPLES {
        let len = fixture.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, _| {
            let uut = String::from(*fixture);
            let uut = std::hint::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("Box<str>", len), &len, |b, _| {
            let uut = Box::<str>::from(*fixture);
            let uut = std::hint::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("Arc<str>", len), &len, |b, _| {
            let uut = std::sync::Arc::<str>::from(*fixture);
            let uut = std::hint::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("StringCow::Owned", len), &len, |b, _| {
            let uut = StringCow::Owned(String::from(*fixture));
            let uut = std::hint::black_box(uut);
            b.iter(|| uut.clone())
        });

        group.bench_with_input(BenchmarkId::new("ArcStr::from", len), &len, |b, _| {
            let uut = arcstr::ArcStr::from(*fixture);
            let uut = std::hint::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("CompactString::new", len), &len, |b, _| {
            let uut = compact_str::CompactString::new(fixture);
            let uut = std::hint::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("EcoString::from", len), &len, |b, _| {
            let uut = ecow::EcoString::from(*fixture);
            let uut = std::hint::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(
            BenchmarkId::new("flexstr::SharedStr::from_ref", len),
            &len,
            |b, _| {
                let uut = flexstr::SharedStr::from_ref(*fixture);
                let uut = std::hint::black_box(uut);
                b.iter(|| uut.clone())
            },
        );
        group.bench_with_input(BenchmarkId::new("HipStr::from", len), &len, |b, _| {
            let uut = hipstr::HipStr::from(*fixture);
            let uut = std::hint::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("ImString::from", len), &len, |b, _| {
            let uut = imstr::ImString::from(*fixture);
            let uut = std::hint::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(BenchmarkId::new("KString::from_ref", len), &len, |b, _| {
            let uut = kstring::KString::from_ref(*fixture);
            let uut = std::hint::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(
            BenchmarkId::new("KString::from_string", len),
            &len,
            |b, _| {
                let uut = kstring::KString::from_string(String::from(*fixture));
                let uut = std::hint::black_box(uut);
                b.iter(|| uut.clone())
            },
        );
        group.bench_with_input(
            BenchmarkId::new("smartstring::String::new", len),
            &len,
            |b, _| {
                let uut = smartstring::alias::String::from(*fixture);
                let uut = std::hint::black_box(uut);
                b.iter(|| uut.clone())
            },
        );
    }
    group.finish();
}

fn bench_clone_static(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone_static");
    group.plot_config(
        criterion::PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic),
    );
    for fixture in fixture::SAMPLES {
        let len = fixture.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("&'static str", len), &len, |b, _| {
            let uut = *fixture;
            let uut = std::hint::black_box(uut);
            b.iter(|| uut.clone())
        });
        group.bench_with_input(
            BenchmarkId::new("StringCow::Borrowed", len),
            &len,
            |b, _| {
                let uut = StringCow::Borrowed(*fixture);
                let uut = std::hint::black_box(uut);
                b.iter(|| uut.clone())
            },
        );
        group.bench_with_input(
            BenchmarkId::new("flexstr::SharedStr::from_static", len),
            &len,
            |b, _| {
                let uut = flexstr::SharedStr::from_static(*fixture);
                let uut = std::hint::black_box(uut);
                b.iter(|| uut.clone())
            },
        );
        group.bench_with_input(
            BenchmarkId::new("hipstr::HipStr::borrowed", len),
            &len,
            |b, _| {
                let uut = hipstr::HipStr::borrowed(*fixture);
                let uut = std::hint::black_box(uut);
                b.iter(|| uut.clone())
            },
        );
        group.bench_with_input(
            BenchmarkId::new("KString::from_static", len),
            &len,
            |b, _| {
                let uut = kstring::KString::from_static(*fixture);
                let uut = std::hint::black_box(uut);
                b.iter(|| uut.clone())
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_clone, bench_clone_static);
criterion_main!(benches);
