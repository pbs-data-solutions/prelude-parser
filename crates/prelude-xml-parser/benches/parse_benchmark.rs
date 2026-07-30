use std::{fs, hint::black_box, io::Write, path::Path, time::Duration};

use criterion::{
    criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion, SamplingMode,
    Throughput,
};
use prelude_xml_parser::{
    parse_site_native_file, parse_site_native_string, parse_subject_native_file,
    parse_subject_native_string, parse_user_native_file, parse_user_native_string,
};
use tempfile::{Builder, NamedTempFile};

const COPIES: [usize; 4] = [1, 10, 100, 1000];

fn load_fixture(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("Could not read fixture {path}: {e}"))
}

fn scaled_xml(fixture: &str, record_tag: &str, copies: usize) -> String {
    let open = format!("<{record_tag} ");
    let close = format!("</{record_tag}>");

    let start = fixture
        .find(&open)
        .unwrap_or_else(|| panic!("No opening {open:?} found in fixture"));
    let end = fixture
        .rfind(&close)
        .map(|i| i + close.len())
        .unwrap_or_else(|| panic!("No closing {close:?} found in fixture"));

    let header = &fixture[..start];
    let records = &fixture[start..end];
    let footer = &fixture[end..];

    let mut out = String::with_capacity(header.len() + records.len() * copies + footer.len());
    out.push_str(header);
    for _ in 0..copies {
        out.push_str(records);
    }
    out.push_str(footer);

    out
}

fn xml_temp_file(contents: &str) -> NamedTempFile {
    let mut file = Builder::new()
        .suffix(".xml")
        .tempfile()
        .unwrap_or_else(|e| panic!("Could not create temp file: {e}"));
    file.write_all(contents.as_bytes())
        .unwrap_or_else(|e| panic!("Could not write temp file: {e}"));
    file.flush()
        .unwrap_or_else(|e| panic!("Could not flush temp file: {e}"));

    file
}

fn configure(group: &mut BenchmarkGroup<'_, criterion::measurement::WallTime>, copies: usize) {
    group.sampling_mode(SamplingMode::Flat);
    if copies >= 100 {
        group.sample_size(10);
        group.warm_up_time(Duration::from_secs(1));
        group.measurement_time(Duration::from_secs(30));
    }
}

fn bench_parser<T, S, F>(
    c: &mut Criterion,
    group_name: &str,
    fixture_path: &str,
    record_tag: &str,
    parse_string: S,
    parse_file: F,
) where
    S: Fn(&str) -> T,
    F: Fn(&Path) -> T,
{
    let fixture = load_fixture(fixture_path);
    let mut group = c.benchmark_group(group_name);

    for copies in COPIES {
        let xml = scaled_xml(&fixture, record_tag, copies);
        let file = xml_temp_file(&xml);
        let path = file.path().to_path_buf();

        configure(&mut group, copies);
        group.throughput(Throughput::Bytes(xml.len() as u64));

        group.bench_with_input(BenchmarkId::new("string", copies), &xml, |b, xml| {
            b.iter(|| parse_string(black_box(xml)))
        });

        group.bench_with_input(BenchmarkId::new("file", copies), &path, |b, path| {
            b.iter(|| parse_file(black_box(path)))
        });
    }

    group.finish();
}

fn bench_parse_subject(c: &mut Criterion) {
    bench_parser(
        c,
        "parse_subject_native",
        "tests/assets/subject_native.xml",
        "patient",
        |xml| {
            parse_subject_native_string(xml).unwrap_or_else(|e| panic!("Parsing failed: {e}"));
        },
        |path| {
            parse_subject_native_file(path).unwrap_or_else(|e| panic!("Parsing failed: {e}"));
        },
    );
}

fn bench_parse_site(c: &mut Criterion) {
    bench_parser(
        c,
        "parse_site_native",
        "tests/assets/site_native.xml",
        "site",
        |xml| {
            parse_site_native_string(xml).unwrap_or_else(|e| panic!("Parsing failed: {e}"));
        },
        |path| {
            parse_site_native_file(path).unwrap_or_else(|e| panic!("Parsing failed: {e}"));
        },
    );
}

fn bench_parse_user(c: &mut Criterion) {
    bench_parser(
        c,
        "parse_user_native",
        "tests/assets/user_native.xml",
        "user",
        |xml| {
            parse_user_native_string(xml).unwrap_or_else(|e| panic!("Parsing failed: {e}"));
        },
        |path| {
            parse_user_native_file(path).unwrap_or_else(|e| panic!("Parsing failed: {e}"));
        },
    );
}

fn bench_parse_subject_real_file(c: &mut Criterion) {
    let Ok(path) = std::env::var("BENCH_XML_FILE") else {
        return;
    };

    let xml = fs::read_to_string(&path).unwrap_or_else(|e| panic!("Could not read {path}: {e}"));

    let mut group = c.benchmark_group("parse_subject_native");
    configure(&mut group, 100);
    group.throughput(Throughput::Bytes(xml.len() as u64));

    group.bench_function("real_string", |b| {
        b.iter(|| {
            parse_subject_native_string(black_box(&xml))
                .unwrap_or_else(|e| panic!("Parsing failed: {e}"));
        })
    });

    group.bench_function("real_file", |b| {
        b.iter(|| {
            parse_subject_native_file(black_box(Path::new(&path)))
                .unwrap_or_else(|e| panic!("Parsing failed: {e}"));
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_parse_subject,
    bench_parse_site,
    bench_parse_user,
    bench_parse_subject_real_file
);
criterion_main!(benches);
