use criterion::{criterion_group, criterion_main, BenchmarkId};
use relay_event_schema::protocol::Event;
use relay_protocol::Annotated;

static TESTS: &[&str] = &["legacy_swift", "cocoa"];

fn bench_from_json(c: &mut criterion::Criterion) {
    for name in TESTS {
        let path = format!("../relay-server/tests/fixtures/payloads/{name}.json");
        let json = std::fs::read_to_string(path).unwrap();

        c.bench_with_input(
            BenchmarkId::new("Annotated::from_json", name),
            &json.as_str(),
            |b, &json| {
                b.iter_with_large_drop(|| Annotated::<Event>::from_json(json));
            },
        );
    }
}

fn bench_to_json(c: &mut criterion::Criterion) {
    for name in TESTS {
        let path = format!("../relay-server/tests/fixtures/payloads/{name}.json");
        let json = std::fs::read_to_string(path).unwrap();

        let event = Annotated::<Event>::from_json(&json).unwrap();

        c.bench_with_input(
            BenchmarkId::new("Annotated::to_json", name),
            &event,
            |b, event| b.iter_with_large_drop(|| event.to_json().unwrap()),
        );
    }
}

criterion_group!(benches, bench_from_json, bench_to_json);
criterion_main!(benches);
