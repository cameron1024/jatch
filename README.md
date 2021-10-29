# Jatch

A Json Patching library in Rust, as per RFC6902

Easily find the difference between 2 JSONs:
```rust
let before = json!({"a": 123});
let after = json!({"a": 123, "b": "hello"});

let patches = diff(before, after);
assert_eq!(patches, vec![Patch::Add {
    value: json!("hello"),
    path: Path::new("/b"),
}]);
```
