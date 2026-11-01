// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! The typed vector API covers every authored row of TC-185, TC-186, TC-187,
//! TC-192 and TC-193. The check reads each authored test case independently of
//! the generator: every `Vector` table row must be exported verbatim, and
//! nothing else may be.

use std::collections::BTreeSet;

use ix_trace_rs::trace;
use quire_specification::{QUALIFICATION_VECTORS, TestCase, vectors_of};

const DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/spec/test-cases");

fn authored(tc: TestCase) -> String {
    let prefix = format!("{}-", tc.id());
    let paths: Vec<_> = std::fs::read_dir(DIR)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(&prefix))
        })
        .collect();
    assert_eq!(
        paths.len(),
        1,
        "{} must have exactly one authored file",
        tc.id()
    );
    std::fs::read_to_string(&paths[0]).unwrap()
}

/// Authored vector rows per test case, in `TestCase::ALL` order.
const ROWS: [usize; 5] = [25, 18, 70, 18, 36];

#[test]
#[trace("TC-185", "TC-186", "TC-187", "TC-192", "TC-193")]
fn api_exports_every_authored_vector_row_exactly_once() {
    let mut total = 0;
    for (tc, rows) in TestCase::ALL.into_iter().zip(ROWS) {
        let text = authored(tc);
        // Independent row discovery: a table whose header starts `| Vector |`.
        let mut expected_ids = Vec::new();
        let mut in_vectors = false;
        for line in text.lines() {
            if !line.starts_with('|') {
                in_vectors = false;
            } else if line.starts_with("| Vector |") {
                in_vectors = true;
            } else if in_vectors && !line.starts_with("| ---") {
                let id = line
                    .trim_start_matches('|')
                    .split('|')
                    .next()
                    .unwrap()
                    .trim();
                expected_ids.push(id.to_owned());
            }
        }
        let got: Vec<&str> = vectors_of(tc).map(|v| v.id).collect();
        assert_eq!(got, expected_ids, "{} ids", tc.id());
        assert_eq!(got.len(), rows, "{} row count", tc.id());
        let unique: BTreeSet<_> = got.iter().collect();
        assert_eq!(unique.len(), got.len(), "{} ids are unique", tc.id());
        for v in vectors_of(tc) {
            assert!(!v.inputs.is_empty() && !v.expected.is_empty(), "{}", v.id);
            // The exported cells are the authored cells, not a re-spelling.
            let row = text
                .lines()
                .find(|l| l.starts_with(&format!("| {} |", v.id)))
                .unwrap();
            assert!(
                row.contains(v.inputs) && row.contains(v.expected),
                "{}",
                v.id
            );
        }
        total += got.len();
    }
    assert_eq!(total, 167);
    assert_eq!(QUALIFICATION_VECTORS.len(), 167);
}
