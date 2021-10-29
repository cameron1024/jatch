use std::cmp::min;

use serde_json::{Map, Value};

use crate::{Patch, Path};

/// Compute the diff between two JSON Documents
///
/// `before` represents the document before the patches are applied
/// `after` represents the document after the patches are applied
///
/// For example:
/// ```rust
/// # use serde_json::json;
/// # use jatch::{diff, apply};
/// let before = json!({"hello": "world"});
/// let after = json!({"hello": "world", "foo": "bar"});
/// let patches = diff(before.clone(), after.clone());
/// let after_again = apply(before, patches).unwrap();
/// assert_eq!(after, after_again);
/// ```
// perhaps this should take &Value 
pub fn diff(before: Value, after: Value) -> Vec<Patch> {
    diff_with_root(before, after, Path::root())
}

fn diff_with_root(before: Value, after: Value, root: Path) -> Vec<Patch> {
    use Value::*;
    match (before, after) {
        (Object(before), Object(after)) => diff_maps(before, after, root),
        (Array(before), Array(after)) => diff_vecs(before, after, root),
        (before, after) => {
            if before == after {
                vec![]
            } else {
                vec![Patch::Replace {
                    path: root,
                    value: after,
                }]
            }
        }
    }
}

fn diff_maps(before: Map<String, Value>, after: Map<String, Value>, root: Path) -> Vec<Patch> {
    if before == after {
        return vec![];
    }

    let keys_to_remove = before.keys().filter(|key| !after.contains_key(*key));
    let keys_to_add = after.keys().filter(|key| !before.contains_key(*key));
    let shared_keys = before.keys().filter(|key| after.contains_key(*key));

    let mut results = vec![];

    results.extend(keys_to_remove.map(|key| Patch::Remove {
        path: root.clone().join(key),
    }));

    results.extend(keys_to_add.map(|key| Patch::Add {
        path: root.clone().join(key),
        value: after.get(key).unwrap().clone(),
    }));

    results.extend(shared_keys.flat_map(|key| {
        diff_with_root(
            before.get(key).unwrap().clone(),
            after.get(key).unwrap().clone(),
            root.clone().join(key),
        )
    }));

    results
}

// this uses a bad approach
// it essentially treats the vec like an object with integer keys
// works well if appending to the list, only the new index has changed
// pretty bad if you insert into the start of the list, since every index will have its corresponding value changed, so gets a patch emitted for it
fn diff_vecs(before: Vec<Value>, after: Vec<Value>, root: Path) -> Vec<Patch> {
    if before == after {
        return vec![];
    }

    let mut results = vec![];

    let shared_indices = 0..(min(before.len(), after.len()));

    for index in shared_indices {
        if before[index] != after[index] {
            results.extend(diff_with_root(
                before[index].clone(),
                after[index].clone(),
                root.clone().join(index.to_string()),
            ));
        }
    }

    if before.len() > after.len() {
        let indices_to_remove = after.len()..before.len();
        results.extend(indices_to_remove.map(|_| Patch::Remove {
            path: root.clone().join(after.len().to_string()), // always use the first index so we can sequentially remove
        }))
    }

    if after.len() > before.len() {
        let indices_to_add = before.len()..after.len();
        results.extend(indices_to_add.map(|i| Patch::Add {
            path: root.clone().join(i.to_string()),
            value: after[i].clone(),
        }))
    }

    results
}

#[cfg(test)]
mod test {

    use serde_json::json;

    use crate::apply;

    use super::*;

    // diff and apply are rough inverses of each other
    // so if "a = diff(b, c)", then "c = apply(b, a)"
    // we can use this to generate some tests
    // note these tests aren't 100% meaningful, since a patch that simply replaces the entire document would be pass the test, but be "incorrect" in a sense
    // they also implicitly depend on `apply` being correct
    fn test_round_trip(before: Value, after: Value) {
        let patches = diff(before.clone(), after.clone());
        let computed_after = apply(before, patches).unwrap();
        assert_eq!(after, computed_after)
    }

    // generate a list of example jsons, we can then take the cartesian cross product and check every round trip works
    fn example_jsons() -> Vec<Value> {
        vec![
            json!(123),
            json!("hello"),
            json!(true),
            json!(null),
            json!({"hello": "world", "deeper": [1, 2, 3]}),
            json!({"hello": "world", "deeper": {"a": "b"}}),
            json!([1, 2, 3, 4, 5]),
            json!([2, 1, 4, 3, 5]),
            json!([1, 2, {"hello": "world"}]),
            json!([[[[[[[1]]]]]]]),
            json!([[[[[[[1, 2]]]]]]]),
            json!([[[1], {"1": "2"}]]),
            json!({
                "name": "foobar",
                "age": 12,
                "pets": [
                    "tom",
                    "dick",
                    "harry",
                ]
            }),
        ]
    }

    #[test]
    fn run_tests() {
        let jsons = example_jsons();
        let pairs = jsons.iter().flat_map(|before| {
            jsons
                .iter()
                .map(move |after| (before.clone(), after.clone()))
        });
        for (before, after) in pairs {
            println!("comparing: {:?} and {:?}", before, after);
            test_round_trip(before, after);
        }
    }

    #[test]
    fn identical_values_should_not_generate_patches() {
        for value in example_jsons() {
            assert_eq!(diff(value.clone(), value.clone()), vec![]);
        }
    }

    #[test]
    fn simple_examples() {
        let add_hello = Patch::Add {
            path: Path::new("/hello"),
            value: json!("world"),
        };
        let add_foo = Patch::Add {
            path: Path::new("/foo"),
            value: json!("bar"),
        };
        let remove_hello = Patch::Remove {
            path: Path::new("/hello"),
        };
        let remove_foo = Patch::Remove {
            path: Path::new("/foo"),
        };

        assert_eq!(
            diff(json!({}), json!({"hello": "world"})),
            vec![add_hello.clone()]
        );
        assert_eq!(
            diff(json!({}), json!({"hello": "world", "foo": "bar"})),
            vec![add_foo, add_hello],
        );
        assert_eq!(
            diff(
                json!({"hello": "world", "foo": "bar"}),
                json!({"hello": "world"})
            ),
            vec![remove_foo.clone()],
        );
        assert_eq!(
            diff(
                json!({"hello": "world", "foo": "bar"}),
                json!({"foo": "bar"})
            ),
            vec![remove_hello.clone()],
        );
        assert_eq!(
            diff(json!({"hello": "world", "foo": "bar"}), json!({})),
            vec![remove_foo, remove_hello],
        );
    }

    #[test]
    fn should_replace_where_appropriate() {
        let before = json!({"hello": "world"});
        let after = json!({"hello": "bar"});
        let patches = diff(before, after);
        assert_eq!(
            patches,
            vec![Patch::Replace {
                path: Path::new("/hello"),
                value: json!("bar"),
            }]
        )
    }
}
