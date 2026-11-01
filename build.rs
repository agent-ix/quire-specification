// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Generates the typed vector table from the authored test-case tables.
//!
//! The authored rows in `spec/test-cases/TC-*.md` are the only source. This
//! build step reads them with `std` alone and emits `static` data into
//! `OUT_DIR`, so consumers iterate the rows through `quire_specification` and
//! never read or copy Markdown themselves.

use std::fmt::Write as _;
use std::path::Path;

/// `(variant, file stem prefix, test case id)` of each exported test case.
const CASES: [(&str, &str, &str); 5] = [
    ("Tc185", "TC-185-", "TC-185"),
    ("Tc186", "TC-186-", "TC-186"),
    ("Tc187", "TC-187-", "TC-187"),
    ("Tc192", "TC-192-", "TC-192"),
    ("Tc193", "TC-193-", "TC-193"),
];

fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let dir = Path::new(&root).join("spec/test-cases");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=spec/test-cases");
    let mut out = String::from("pub static QUALIFICATION_VECTORS: &[QualificationVector] = &[\n");
    for (variant, prefix, tc) in CASES {
        let mut paths: Vec<_> = std::fs::read_dir(&dir)
            .expect("read spec/test-cases")
            .map(|e| e.expect("dir entry").path())
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with(prefix) && n.ends_with(".md"))
            })
            .collect();
        assert_eq!(paths.len(), 1, "{tc}: expected exactly one authored file");
        let path = paths.remove(0);
        println!("cargo:rerun-if-changed={}", path.display());
        let text = std::fs::read_to_string(&path).expect("read test case");
        let mut in_vectors = false;
        for line in text.lines() {
            let Some(cells) = split_row(line) else {
                in_vectors = false;
                continue;
            };
            if cells.first().map(String::as_str) == Some("Vector") {
                assert_eq!(cells.len(), 3, "{tc}: vector table must have 3 columns");
                in_vectors = true;
                continue;
            }
            if !in_vectors || cells.iter().all(|c| c.chars().all(|ch| ch == '-')) {
                continue;
            }
            assert_eq!(cells.len(), 3, "{tc}: row `{line}` must have 3 cells");
            writeln!(
                out,
                "    QualificationVector {{ test_case: TestCase::{variant}, id: {:?}, inputs: {:?}, expected: {:?} }},",
                cells[0], cells[1], cells[2]
            )
            .expect("write");
        }
    }
    out.push_str("];\n");
    let dest = Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR")).join("vectors.rs");
    std::fs::write(dest, out).expect("write vectors.rs");
}

/// Splits a Markdown table row into trimmed cells, honouring code spans and
/// `\|` escapes. Returns `None` for a line that is not a table row.
fn split_row(line: &str) -> Option<Vec<String>> {
    let line = line.trim();
    let inner = line.strip_prefix('|')?.strip_suffix('|')?;
    let mut cells = Vec::new();
    let mut cur = String::new();
    let mut in_code = false;
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '`' => {
                in_code = !in_code;
                cur.push(c);
            }
            '\\' if chars.peek() == Some(&'|') => {
                cur.push('|');
                chars.next();
            }
            '|' if !in_code => {
                cells.push(cur.trim().to_owned());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    cells.push(cur.trim().to_owned());
    Some(cells)
}
