use criterion::{criterion_group, criterion_main, Criterion};
use jatch::{apply, Patch, Path};
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

fn deep_insert(c: &mut Criterion) {
    let json = deep_json(JSON_DEPTH);
    let patches = vec![Patch::Add {
        path: Path::new("/deeper".repeat(JSON_DEPTH)),
        value: json!("value"),
    }];
    c.bench_function("deep_insert", |b| {
        b.iter(|| apply(json.clone(), patches.clone()))
    });
}

fn append_long_array(c: &mut Criterion) {
    let mut json = json!([]);
    let vec = json.as_array_mut().unwrap();
    vec.extend(vec![json!(1); 10000]);

    let patches = vec![Patch::Add {
        path: Path::new("/-"),
        value: json!(123),
    }];

    c.bench_function("append_long_array", |b| {
        b.iter(|| apply(json.clone(), patches.clone()))
    });
}

criterion_group!(benches, deep_insert, append_long_array);
criterion_main!(benches);
