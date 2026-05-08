use std::{fs, hint::black_box, path::Path, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use prelude_xml_parser::{parse_subject_native_file, parse_subject_native_string};

fn bench_parse_subject_string(c: &mut Criterion) {
    let path = std::env::var("BENCH_XML_FILE")
        .expect("Set BENCH_XML_FILE to the path of the subject native XML file to benchmark");
    let xml = fs::read_to_string(&path).unwrap_or_else(|e| panic!("Could not read {path}: {e}"));

    let mut group = c.benchmark_group("parse_subject_native");
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(30));

    group.bench_function("subject_string", |b| {
        b.iter(|| parse_subject_native_string(black_box(&xml)).unwrap())
    });

    group.finish();
}

fn bench_parse_subject_file(c: &mut Criterion) {
    let path = std::env::var("BENCH_XML_FILE")
        .expect("Set BENCH_XML_FILE to the path of the subject native XML file to benchmark");

    let mut group = c.benchmark_group("parse_subject_native");
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(30));

    group.bench_function("subject_file", |b| {
        b.iter(|| parse_subject_native_file(black_box(Path::new(&path))).unwrap())
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_parse_subject_string,
    bench_parse_subject_file
);
criterion_main!(benches);
