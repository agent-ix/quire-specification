// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! STD-609: the executable mutation manifest of the protocol node vectors
//! (TC-946, TC-947, TC-948). The manifest must follow its schema, name every
//! vector exactly once as a row, a later row or a pending question, edit a
//! base that exists and is schema-valid, and carry loci that its own edits
//! reach.

use ix_trace_rs::trace;
use serde_json::Value;
use std::collections::BTreeSet;

const MANIFEST: &str =
    include_str!("../proposals/checked-package-v2/fixtures/protocol-node-mutation-manifest.json");
const MANIFEST_SCHEMA: &str =
    include_str!("../proposals/checked-package-v2/protocol-node-mutation-manifest.schema.json");
const VECTORS: &str = include_str!("../proposals/checked-package-v2/fixtures/protocol-node-vectors.json");
const BASE: &str = include_str!("../proposals/checked-package-v2/fixtures/protocol-node-base.json");
const PACKAGE_SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");

fn parse(text: &str) -> Value {
    serde_json::from_str(text).expect("authored fixture is valid JSON")
}

fn unescape(segment: &str) -> String {
    segment.replace("~1", "/").replace("~0", "~")
}

/// Applies the RFC 6902 `add`, `remove` and `replace` operations a row uses.
fn apply(base: &Value, edit: &[Value]) -> Value {
    let mut package = base.clone();
    for op in edit {
        let path = op["path"].as_str().expect("path");
        let (parent, last) = path.rsplit_once('/').expect("absolute pointer");
        let last = unescape(last);
        let target = package.pointer_mut(parent).unwrap_or_else(|| panic!("{path}: parent exists"));
        match (op["op"].as_str().expect("op"), target) {
            ("remove", Value::Object(map)) => {
                map.remove(&last).unwrap_or_else(|| panic!("{path}: member exists"));
            }
            ("remove", Value::Array(items)) => {
                items.remove(last.parse::<usize>().expect("index"));
            }
            ("replace" | "add", Value::Object(map)) => {
                map.insert(last, op["value"].clone());
            }
            ("replace", Value::Array(items)) => {
                items[last.parse::<usize>().expect("index")] = op["value"].clone();
            }
            ("add", Value::Array(items)) => {
                let at = if last == "-" { items.len() } else { last.parse().expect("index") };
                items.insert(at, op["value"].clone());
            }
            (other, _) => panic!("{path}: unsupported {other}"),
        }
    }
    package
}

fn declaration_name(node: &Value) -> String {
    match node.get("declaration") {
        Some(declaration) => declaration["qualified_name"]
            .as_array()
            .expect("qualified name")
            .iter()
            .map(|s| s.as_str().expect("segment"))
            .collect::<Vec<_>>()
            .join("::"),
        None => serde_json::to_string(&node["node_id"]["path"]).expect("path"),
    }
}

fn ids_of(list: &Value) -> Vec<(String, String)> {
    list.as_array()
        .expect("array")
        .iter()
        .map(|r| (r["test_case"].as_str().unwrap().to_owned(), r["id"].as_str().unwrap().to_owned()))
        .collect()
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn the_manifest_follows_its_schema() {
    let schema = jsonschema::validator_for(&parse(MANIFEST_SCHEMA)).expect("manifest schema is valid");
    let manifest = parse(MANIFEST);
    let errors: Vec<String> = schema
        .iter_errors(&manifest)
        .map(|e| format!("{e} at {}", e.instance_path()))
        .take(5)
        .collect();
    assert!(errors.is_empty(), "{errors:?}");
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn every_vector_is_a_row_a_later_row_or_a_pending_question_exactly_once() {
    let manifest = parse(MANIFEST);
    let vectors = parse(VECTORS);
    let expected: Vec<(String, String)> = ids_of(&vectors["cases"]);
    assert_eq!(expected.len(), 406, "the prose corpus is 406 vectors");
    let mut seen = BTreeSet::new();
    for list in ["rows", "later", "pending"] {
        for entry in ids_of(&manifest[list]) {
            assert!(seen.insert(entry.clone()), "{entry:?} appears in more than one place");
        }
    }
    let expected: BTreeSet<_> = expected.into_iter().collect();
    assert_eq!(seen, expected, "manifest partitions the vectors, with the same test case per id");
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn every_row_edits_a_base_that_exists_and_is_schema_valid() {
    let manifest = parse(MANIFEST);
    let base = parse(BASE);
    assert_eq!(manifest["base"]["fixture"], "protocol-node-base.json");
    let validator = jsonschema::validator_for(&parse(PACKAGE_SCHEMA)).expect("I04 schema is valid");
    assert!(validator.is_valid(&base), "the base package is schema-valid");
    for row in manifest["rows"].as_array().unwrap() {
        assert_eq!(row["base"], manifest["base"]["fixture"], "{}: base ref", row["id"]);
    }
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn every_edit_reaches_the_node_it_names_and_its_locus_resolves() {
    let manifest = parse(MANIFEST);
    let base = parse(BASE);
    let validator = jsonschema::validator_for(&parse(PACKAGE_SCHEMA)).expect("I04 schema is valid");
    let nodes = base["semantic_graph"]["nodes"].as_array().expect("nodes");
    for row in manifest["rows"].as_array().unwrap() {
        let id = row["id"].as_str().unwrap();
        let edit = row["edit"].as_array().unwrap();
        for op in edit {
            let path = op["path"].as_str().unwrap();
            if let Some(rest) = path.strip_prefix("/semantic_graph/nodes/") {
                let index: usize = rest.split('/').next().unwrap().parse().expect("node index");
                assert_eq!(
                    op["node"].as_str().unwrap_or_default(),
                    declaration_name(&nodes[index]),
                    "{id}: {path} enters the node it names"
                );
            }
        }
        let edited = apply(&base, edit);
        let first_error = validator
            .iter_errors(&edited)
            .next()
            .map(|e| e.instance_path().to_string());
        let expect = &row["expect"];
        match expect["decision"].as_str().unwrap() {
            "refuse" if expect["stage"] == "schema" => {
                assert_eq!(first_error.as_deref(), expect["locus"].as_str(), "{id}: wire locus is the first schema error path");
            }
            _ => assert_eq!(first_error, None, "{id}: the edit passes the schema and reaches the reader"),
        }
        if expect["decision"] == "refuse" && expect["stage"] != "schema" {
            let locus = expect["locus"].as_str().unwrap();
            assert!(edited.pointer(locus).is_some(), "{id}: locus {locus} resolves in the edited package");
        }
    }
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn refusal_rows_carry_a_code_and_admission_rows_nothing_more() {
    let manifest = parse(MANIFEST);
    let mut counts = (0, 0);
    for row in manifest["rows"].as_array().unwrap() {
        match row["class"].as_str().unwrap() {
            "refuse" => {
                assert!(row["expect"]["code"].is_string(), "{}", row["id"]);
                counts.0 += 1;
            }
            "admit" => {
                assert_eq!(row["expect"], serde_json::json!({"decision": "admit"}));
                counts.1 += 1;
            }
            other => panic!("{}: class {other} has no rows in PR 1", row["id"]),
        }
    }
    assert!(counts.0 > 0 && counts.1 > 0, "both outcomes are executed");
}
