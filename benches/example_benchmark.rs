use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use jatch::{apply, walk, Patch, Path};
use serde_json::{json, Value};

fn deep_json(depth: usize) -> Value {
    if depth == 0 {
        json!(null)
    } else {
        json!({
            "null": null,
            "number": 123,
            "string": "hello",
            "array": [1, 2, 3],
            "obj": {"foo": "bar"},
            "deeper": deep_json(depth - 1)
        })
    }
}

const JSON_DEPTH: usize = 16;

fn deep_walk(c: &mut Criterion) {
    c.bench_function("deep_walk", |c| {
        c.iter_batched(
            || {
                (
                    deep_json(JSON_DEPTH),
                    Path::new("/deeper".repeat(JSON_DEPTH)).unwrap(),
                )
            },
            |(value, path)| {
                walk(&value, path).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
}

fn deep_insert(c: &mut Criterion) {
    let setup = || {
        let json = deep_json(JSON_DEPTH);
        let patches = vec![Patch::Add {
            path: Path::new("/deeper".repeat(JSON_DEPTH)).unwrap(),
            value: json!("value"),
        }];
        (json, patches)
    };
    c.bench_function("deep_insert", |b| {
        b.iter_batched(
            setup,
            |(value, patches)| apply(value, patches),
            BatchSize::SmallInput,
        )
    });
}

fn append_long_array(c: &mut Criterion) {
    let setup = || {
        let mut json = json!([]);
        let vec = json.as_array_mut().unwrap();
        vec.extend(vec![json!(1); 10000]);

        (
            json,
            vec![Patch::Add {
                path: Path::new("/-").unwrap(),
                value: json!(123),
            }],
        )
    };

    c.bench_function("append_long_array", |b| {
        b.iter_batched(
            setup,
            |(json, patches)| apply(json, patches),
            BatchSize::SmallInput,
        )
    });
}

fn insert_start_long_array(c: &mut Criterion) {
    let setup = || {
        let mut json = json!([]);
        let vec = json.as_array_mut().unwrap();
        vec.extend(vec![json!(1); 10000]);
        (
            json,
            vec![Patch::Add {
                path: Path::new("/0").unwrap(),
                value: json!(123),
            }],
        )
    };
    c.bench_function("insert_start_long_array", |b| {
        b.iter_batched(
            setup,
            |(value, patches)| apply(value, patches),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    deep_walk,
    deep_insert,
    append_long_array,
    insert_start_long_array
);
criterion_main!(benches);
