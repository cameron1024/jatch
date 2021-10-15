#![allow(dead_code)]

use serde_json::Value;
#[derive(Debug, PartialEq, Eq)]
pub enum VecDifference {
    Addition(Value),
    Removal(Value),
}

pub fn diff(before: &[Value], after: &[Value]) -> Vec<VecDifference> {
    let lcs_grid = longest_common_subsequence(before, after);
    let mut results = vec![];

    let mut i = before.len();
    let mut j = after.len();

    while i != 0 || j != 0 {
        if i == 0 {
            results.push(VecDifference::Addition(after[j - 1].clone()));
            j -= 1;
        } else if j == 0 {
            results.push(VecDifference::Removal(after[i - 1].clone()));
            i -= 1
        } else if before[i - 1] == after[j - 1] {
            i -= 1;
            j -= 1;
        } else if lcs_grid[i - 1][j] <= lcs_grid[i][j - 1] {
            results.push(VecDifference::Addition(after[j - 1].clone()));
            j -= 1;
        } else {
            results.push(VecDifference::Removal(before[i - 1].clone()));
            i -= 1;
        }
    }

    results
}

fn longest_common_subsequence<T: Eq>(l1: &[T], l2: &[T]) -> Vec<Vec<usize>> {
    let n = l1.len();
    let m = l2.len();

    let mut result = vec![vec![usize::MAX; m + 1]; n + 1];

    for i in 0..(n + 1) {
        for j in 0..(m + 1) {
            result[i][j] = if i == 0 || j == 0 {
                0
            } else if l1[i - 1] == l2[j - 1] {
                1 + result[i - 1][j - 1]
            } else {
                std::cmp::max(result[i - 1][j], result[i][j - 1])
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use test_case::test_case;

    #[test_case("hello", "hello" => 5)]
    #[test_case("hello", "helloasdf" => 5)]
    #[test_case("hello", "" => 0)]
    #[test_case("", "" => 0)]
    fn should_compute_lcs_length(s1: &'static str, s2: &'static str) -> usize {
        let result = longest_common_subsequence(s1.as_bytes(), s2.as_bytes());
        result[s1.len()][s2.len()]
    }

    #[test]
    fn should_compute_diffs() {
        let before = vec![json!(1), json!(2)];
        let after = vec![json!(1)];

        let changes = diff(&before, &after);
        assert_eq!(changes, vec![VecDifference::Removal(json!(2))]);
    }
}
